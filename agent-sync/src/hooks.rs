use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde_json::{Map, Value};
use walkdir::WalkDir;

use crate::config::Config;
use crate::install::{self, InstallMode, InstallOutcome};
use crate::library::LibraryItem;
use crate::state::{InstalledPath, State};
use crate::target::Target;

const MANAGED_PREFIX: &str = "agent-sync:";
const HOOK_TARGETS: [Target; 3] = [Target::Claude, Target::Cursor, Target::Pi];

pub fn sync(
    config: &Config,
    packs: &[&LibraryItem],
    old_state: &State,
    dry_run: bool,
) -> Result<Vec<InstalledPath>> {
    let mut installed = Vec::new();

    for target in HOOK_TARGETS {
        let mut target_entries: BTreeMap<String, Vec<Map<String, Value>>> = BTreeMap::new();
        for pack in packs {
            if pack.manifest.excludes(target) {
                continue;
            }
            let entries = pack
                .manifest
                .hooks
                .get(&target)
                .cloned()
                .unwrap_or_default();
            if entries.is_empty() {
                continue;
            }

            let scripts = install_scripts(config, pack, target, old_state, dry_run)?;
            let rewrites = scripts
                .iter()
                .map(|(source, destination, _)| (source.clone(), destination.clone()))
                .collect::<Vec<_>>();
            installed.extend(scripts.into_iter().map(|(source, destination, mode)| {
                InstalledPath::new(
                    source,
                    target.id(),
                    destination,
                    mode.as_str(),
                    "hooks",
                    &pack.name,
                )
            }));

            for (event, event_entries) in entries {
                let output = target_entries.entry(event.clone()).or_default();
                for (ordinal, mut entry) in event_entries.into_iter().enumerate() {
                    rewrite_commands(&mut entry, &rewrites);
                    entry.insert(
                        "_as".to_owned(),
                        Value::String(format!(
                            "agent-sync:{}:{}:{event}:{ordinal}",
                            pack.name, pack.manifest.version
                        )),
                    );
                    output.push(entry);
                }
            }
        }

        if target == Target::Pi {
            installed.extend(sync_pi_extensions(config, &target_entries, dry_run)?);
        } else {
            merge_config(config, target, target_entries, dry_run)?;
        }
    }

    Ok(installed)
}

pub fn verify(config: &Config, packs: &[&LibraryItem]) -> Result<bool> {
    let mut valid = true;
    for target in HOOK_TARGETS {
        if target == Target::Pi {
            if !verify_pi(config, packs)? {
                valid = false;
            }
            continue;
        }

        let expected = expected_managed(config, packs, target)?;
        let config_path = target
            .hooks_config(&config.target_home)
            .context("supported hook target must have a config path")?;
        let value = read_config(&config_path)?;
        let actual = managed_entries(&value)?;
        if actual != expected {
            eprintln!(
                "ERROR hooks {target}: managed entries differ in {}",
                config_path.display()
            );
            valid = false;
        }

        for pack in packs {
            if pack.manifest.excludes(target)
                || pack
                    .manifest
                    .hooks
                    .get(&target)
                    .is_none_or(BTreeMap::is_empty)
            {
                continue;
            }
            for (_, destination, _) in script_plan(config, pack, target)? {
                if !install::path_exists(&destination) {
                    eprintln!(
                        "ERROR hooks {target}: missing script {}",
                        destination.display()
                    );
                    valid = false;
                }
            }
        }
    }
    Ok(valid)
}

pub fn remove_managed(config: &Config) -> Result<()> {
    for target in HOOK_TARGETS {
        if target == Target::Pi {
            clear_pi_managed(config)?;
        } else {
            merge_config(config, target, BTreeMap::new(), false)?;
        }
    }
    Ok(())
}

pub fn expected_script_paths(config: &Config, packs: &[&LibraryItem]) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for target in HOOK_TARGETS {
        for pack in packs {
            if pack.manifest.excludes(target)
                || pack
                    .manifest
                    .hooks
                    .get(&target)
                    .is_none_or(BTreeMap::is_empty)
            {
                continue;
            }
            paths.extend(
                script_plan(config, pack, target)?
                    .into_iter()
                    .map(|(_, destination, _)| destination),
            );
        }
        if target == Target::Pi {
            let wrappers = pi_wrapper_plan(config, packs)?;
            if !wrappers.is_empty() {
                for (wrapper, _) in wrappers {
                    paths.push(wrapper);
                }
                if let Some(registry) = target.hooks_managed_registry(&config.target_home) {
                    paths.push(registry);
                }
            }
        }
    }
    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn sync_pi_extensions(
    config: &Config,
    entries: &BTreeMap<String, Vec<Map<String, Value>>>,
    dry_run: bool,
) -> Result<Vec<InstalledPath>> {
    let hooks_dir = Target::Pi
        .hooks_dir(&config.target_home)
        .context("Pi must have an extensions directory")?;
    let registry_path = Target::Pi
        .hooks_managed_registry(&config.target_home)
        .context("Pi must have a managed registry path")?;

    if entries.is_empty() && !registry_path.exists() {
        return Ok(Vec::new());
    }

    let registry_entries = pi_registry_entries(entries)?;
    let count = registry_entries.len();
    if dry_run {
        println!(
            "PLAN hooks/pi -> {} ({count} TypeScript wrappers)",
            hooks_dir.display()
        );
        return Ok(Vec::new());
    }

    clear_pi_managed(config)?;
    fs::create_dir_all(&hooks_dir)
        .with_context(|| format!("create Pi extensions dir {}", hooks_dir.display()))?;

    let mut installed = Vec::new();
    for entry in &registry_entries {
        let tag = entry
            .get("_as")
            .and_then(Value::as_str)
            .unwrap_or("agent-sync:unknown");
        let event = entry
            .get("event")
            .and_then(Value::as_str)
            .context("Pi registry entry missing event")?;
        let command = entry
            .get("command")
            .and_then(Value::as_str)
            .context("Pi registry entry missing command")?;
        let wrapper_name = entry
            .get("wrapper")
            .and_then(Value::as_str)
            .context("Pi registry entry missing wrapper")?;
        let wrapper_path = hooks_dir.join(wrapper_name);
        let body = pi_ts_wrapper(event, command, tag);
        fs::write(&wrapper_path, body)
            .with_context(|| format!("write Pi wrapper {}", wrapper_path.display()))?;
        println!("SYNC hooks/pi -> {} (typescript)", wrapper_path.display());
        installed.push(InstalledPath::new(
            PathBuf::from(format!("hooks/pi/{wrapper_name}")),
            Target::Pi.id(),
            wrapper_path,
            InstallMode::Copy.as_str(),
            "hooks",
            "pi-extension",
        ));
    }

    let registry = Value::Object(Map::from_iter([
        ("version".to_owned(), Value::from(1)),
        ("entries".to_owned(), Value::Array(registry_entries)),
    ]));
    write_json_atomic(&registry_path, &registry)?;
    println!("SYNC hooks/pi registry -> {}", registry_path.display());
    installed.push(InstalledPath::new(
        PathBuf::from("hooks/pi/.agent-sync-managed.json"),
        Target::Pi.id(),
        registry_path,
        InstallMode::Copy.as_str(),
        "hooks",
        "pi-extension",
    ));
    Ok(installed)
}

fn verify_pi(config: &Config, packs: &[&LibraryItem]) -> Result<bool> {
    let mut valid = true;
    let expected = expected_pi_registry(config, packs)?;
    let registry_path = Target::Pi
        .hooks_managed_registry(&config.target_home)
        .context("Pi must have a managed registry path")?;

    if expected.is_empty() {
        if registry_path.exists() {
            let value = read_config(&registry_path)?;
            let entries = value
                .get("entries")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            if !entries.is_empty() {
                eprintln!(
                    "ERROR hooks pi: unexpected managed entries in {}",
                    registry_path.display()
                );
                valid = false;
            }
        }
        return Ok(valid);
    }

    if !registry_path.exists() {
        eprintln!(
            "ERROR hooks pi: missing managed registry {}",
            registry_path.display()
        );
        return Ok(false);
    }

    let actual = read_config(&registry_path)?;
    let actual_entries = actual
        .get("entries")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    if actual_entries != expected {
        eprintln!(
            "ERROR hooks pi: managed entries differ in {}",
            registry_path.display()
        );
        valid = false;
    }

    let hooks_dir = Target::Pi
        .hooks_dir(&config.target_home)
        .context("Pi must have an extensions directory")?;
    for entry in &expected {
        if let Some(wrapper) = entry.get("wrapper").and_then(Value::as_str) {
            let path = hooks_dir.join(wrapper);
            if !install::path_exists(&path) {
                eprintln!("ERROR hooks pi: missing wrapper {}", path.display());
                valid = false;
            }
        }
        if let Some(command) = entry.get("command").and_then(Value::as_str) {
            if command.contains('/') && !install::path_exists(Path::new(command)) {
                eprintln!("ERROR hooks pi: missing script {command}");
                valid = false;
            }
        }
    }

    for pack in packs {
        if pack.manifest.excludes(Target::Pi)
            || pack
                .manifest
                .hooks
                .get(&Target::Pi)
                .is_none_or(BTreeMap::is_empty)
        {
            continue;
        }
        for (_, destination, _) in script_plan(config, pack, Target::Pi)? {
            if !install::path_exists(&destination) {
                eprintln!(
                    "ERROR hooks pi: missing script {}",
                    destination.display()
                );
                valid = false;
            }
        }
    }
    Ok(valid)
}

fn expected_pi_registry(config: &Config, packs: &[&LibraryItem]) -> Result<Vec<Value>> {
    let mut target_entries: BTreeMap<String, Vec<Map<String, Value>>> = BTreeMap::new();
    for pack in packs {
        if pack.manifest.excludes(Target::Pi) {
            continue;
        }
        let Some(events) = pack.manifest.hooks.get(&Target::Pi) else {
            continue;
        };
        let scripts = script_plan(config, pack, Target::Pi)?;
        let rewrites = scripts
            .into_iter()
            .map(|(source, destination, _)| (source, destination))
            .collect::<Vec<_>>();
        for (event, event_entries) in events {
            let output = target_entries.entry(event.clone()).or_default();
            for (ordinal, entry) in event_entries.iter().enumerate() {
                let mut entry = entry.clone();
                rewrite_commands(&mut entry, &rewrites);
                entry.insert(
                    "_as".to_owned(),
                    Value::String(format!(
                        "agent-sync:{}:{}:{event}:{ordinal}",
                        pack.name, pack.manifest.version
                    )),
                );
                output.push(entry);
            }
        }
    }
    pi_registry_entries(&target_entries)
}

fn pi_registry_entries(
    entries: &BTreeMap<String, Vec<Map<String, Value>>>,
) -> Result<Vec<Value>> {
    let mut registry = Vec::new();
    for (event, event_entries) in entries {
        for entry in event_entries {
            let tag = entry
                .get("_as")
                .and_then(Value::as_str)
                .context("Pi hook entry missing _as tag")?
                .to_owned();
            let command = entry
                .get("command")
                .and_then(Value::as_str)
                .with_context(|| format!("Pi hook {tag} missing command"))?
                .to_owned();
            let (pack, ordinal) = parse_as_pack_ordinal(&tag)
                .with_context(|| format!("parse Pi hook tag {tag}"))?;
            let wrapper_name = format!("as-{pack}-{event}-{ordinal}.ts");
            registry.push(Value::Object(Map::from_iter([
                ("_as".to_owned(), Value::String(tag)),
                ("event".to_owned(), Value::String(event.clone())),
                ("wrapper".to_owned(), Value::String(wrapper_name)),
                ("command".to_owned(), Value::String(command)),
            ])));
        }
    }
    Ok(registry)
}

fn parse_as_pack_ordinal(tag: &str) -> Result<(String, String)> {
    // agent-sync:<pack>:<version>:<event>:<ordinal>
    let rest = tag
        .strip_prefix(MANAGED_PREFIX)
        .with_context(|| format!("tag {tag} missing {MANAGED_PREFIX} prefix"))?;
    let mut parts = rest.split(':');
    let pack = parts
        .next()
        .filter(|part| !part.is_empty())
        .context("tag missing pack")?
        .to_owned();
    let _version = parts.next().context("tag missing version")?;
    let mut remainder = parts.collect::<Vec<_>>();
    let ordinal = remainder
        .pop()
        .filter(|part| !part.is_empty())
        .context("tag missing ordinal")?
        .to_owned();
    if remainder.is_empty() {
        bail!("tag {tag} missing event");
    }
    Ok((pack, ordinal))
}

fn pi_wrapper_plan(
    config: &Config,
    packs: &[&LibraryItem],
) -> Result<Vec<(PathBuf, String)>> {
    let hooks_dir = Target::Pi
        .hooks_dir(&config.target_home)
        .context("Pi must have an extensions directory")?;
    expected_pi_registry(config, packs)?
        .into_iter()
        .filter_map(|entry| {
            let wrapper = entry.get("wrapper")?.as_str()?.to_owned();
            let tag = entry.get("_as")?.as_str()?.to_owned();
            Some((hooks_dir.join(wrapper), tag))
        })
        .map(Ok)
        .collect()
}

fn clear_pi_managed(config: &Config) -> Result<()> {
    let registry_path = Target::Pi
        .hooks_managed_registry(&config.target_home)
        .context("Pi must have a managed registry path")?;
    let hooks_dir = Target::Pi
        .hooks_dir(&config.target_home)
        .context("Pi must have an extensions directory")?;
    if registry_path.exists() {
        let value = read_config(&registry_path)?;
        if let Some(entries) = value.get("entries").and_then(Value::as_array) {
            for entry in entries {
                if let Some(wrapper) = entry.get("wrapper").and_then(Value::as_str) {
                    install::remove_path(&hooks_dir.join(wrapper))?;
                }
            }
        }
        install::remove_path(&registry_path)?;
    }
    Ok(())
}

fn pi_ts_wrapper(event: &str, command: &str, tag: &str) -> String {
    // Escape for embedding in a TypeScript single-quoted string.
    let command_lit = command.replace('\\', "\\\\").replace('\'', "\\'");
    let tag_lit = tag.replace('\\', "\\\\").replace('\'', "\\'");
    let event_lit = event.replace('\\', "\\\\").replace('\'', "\\'");
    format!(
        r#"// Generated by agent-sync — do not edit.
// {tag_lit}
import type {{ ExtensionAPI }} from "@earendil-works/pi-coding-agent";
import {{ spawn }} from "node:child_process";

export default function (pi: ExtensionAPI) {{
  pi.on('{event_lit}', async () => {{
    await new Promise<void>((resolve, reject) => {{
      const child = spawn('{command_lit}', {{ stdio: "inherit", shell: true }});
      child.on("error", reject);
      child.on("close", (code) => {{
        if (code === 0) resolve();
        else reject(new Error(`agent-sync hook exited ${{code}}`));
      }});
    }});
  }});
}}
"#
    )
}

fn install_scripts(
    config: &Config,
    pack: &LibraryItem,
    target: Target,
    old_state: &State,
    dry_run: bool,
) -> Result<Vec<(PathBuf, PathBuf, InstallMode)>> {
    let preferred = if target == Target::Cursor {
        InstallMode::Copy
    } else {
        InstallMode::Symlink
    };
    let mut installed = Vec::new();
    for (source, destination, _) in script_plan(config, pack, target)? {
        let outcome = install::install(
            &source,
            &destination,
            preferred,
            old_state.owns(&destination),
            dry_run,
        )?;
        match outcome {
            InstallOutcome::Installed(mode) => {
                println!(
                    "{} hooks/{} -> {} ({})",
                    if dry_run { "PLAN" } else { "SYNC" },
                    pack.name,
                    destination.display(),
                    mode.as_str()
                );
                installed.push((source, destination, mode));
            }
            InstallOutcome::SkippedForeign(reason) => bail!(
                "refusing hook script destination {}: {reason}",
                destination.display()
            ),
        }
    }
    Ok(installed)
}

fn script_plan(
    config: &Config,
    pack: &LibraryItem,
    target: Target,
) -> Result<Vec<(PathBuf, PathBuf, InstallMode)>> {
    let hooks_dir = target
        .hooks_dir(&config.target_home)
        .context("supported hook target must have a scripts directory")?;
    let mut scripts = Vec::new();
    for entry in WalkDir::new(&pack.path).follow_links(true) {
        let entry = entry.with_context(|| format!("walk Hook pack {}", pack.path.display()))?;
        if !entry.file_type().is_file() || !is_hook_script(entry.path()) {
            continue;
        }
        let relative = entry.path().strip_prefix(&pack.path)?;
        let flattened = relative.to_string_lossy().replace(['/', '\\'], "-");
        let destination = hooks_dir.join(format!("as-{}-{flattened}", pack.name));
        let mode = if target == Target::Cursor {
            InstallMode::Copy
        } else {
            InstallMode::Symlink
        };
        scripts.push((entry.path().to_path_buf(), destination, mode));
    }
    scripts.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(scripts)
}

fn is_hook_script(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("sh" | "py" | "js" | "mjs" | "ts")
    )
}

fn rewrite_commands(entry: &mut Map<String, Value>, rewrites: &[(PathBuf, PathBuf)]) {
    rewrite_value(&mut Value::Object(entry.clone()), rewrites, entry);
}

fn rewrite_value(
    value: &mut Value,
    rewrites: &[(PathBuf, PathBuf)],
    root: &mut Map<String, Value>,
) {
    fn visit(value: &mut Value, rewrites: &[(PathBuf, PathBuf)], key: Option<&str>) {
        match value {
            Value::Object(object) => {
                for (name, child) in object {
                    visit(child, rewrites, Some(name));
                }
            }
            Value::Array(array) => {
                for child in array {
                    visit(child, rewrites, key);
                }
            }
            Value::String(command) if key == Some("command") => {
                for (source, destination) in rewrites {
                    let Some(file_name) = source.file_name().and_then(|name| name.to_str()) else {
                        continue;
                    };
                    let explicit_relative = format!("./{file_name}");
                    if command.contains(&explicit_relative) {
                        *command =
                            command.replace(&explicit_relative, &destination.to_string_lossy());
                    } else if command == file_name {
                        *command = destination.to_string_lossy().into_owned();
                    }
                }
            }
            _ => {}
        }
    }

    visit(value, rewrites, None);
    if let Value::Object(object) = std::mem::take(value) {
        *root = object;
    }
}

fn merge_config(
    config: &Config,
    target: Target,
    entries: BTreeMap<String, Vec<Map<String, Value>>>,
    dry_run: bool,
) -> Result<()> {
    let path = target
        .hooks_config(&config.target_home)
        .context("supported hook target must have a config path")?;
    if entries.is_empty() && !path.exists() {
        return Ok(());
    }
    if dry_run {
        let count = entries.values().map(Vec::len).sum::<usize>();
        println!(
            "PLAN hooks/{target} -> {} ({count} entries)",
            path.display()
        );
        return Ok(());
    }

    let mut root = read_config(&path)?;
    let root_object = root
        .as_object_mut()
        .context("hook config root must be a JSON object")?;
    if target == Target::Cursor && !root_object.contains_key("version") {
        root_object.insert("version".to_owned(), Value::from(1));
    }
    let hooks = root_object
        .entry("hooks")
        .or_insert_with(|| Value::Object(Map::new()))
        .as_object_mut()
        .context("hook config 'hooks' must be a JSON object")?;

    for existing in hooks.values_mut() {
        let array = existing
            .as_array_mut()
            .context("hook event value must be an array")?;
        array.retain(|entry| {
            entry
                .get("_as")
                .and_then(Value::as_str)
                .is_none_or(|tag| !tag.starts_with(MANAGED_PREFIX))
        });
    }
    hooks.retain(|_, value| value.as_array().is_none_or(|array| !array.is_empty()));

    for (event, additions) in entries {
        let array = hooks
            .entry(event)
            .or_insert_with(|| Value::Array(Vec::new()))
            .as_array_mut()
            .context("hook event value must be an array")?;
        array.extend(additions.into_iter().map(Value::Object));
    }
    write_json_atomic(&path, &root)
}

fn expected_managed(
    config: &Config,
    packs: &[&LibraryItem],
    target: Target,
) -> Result<BTreeMap<String, Vec<Value>>> {
    let mut managed: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for pack in packs {
        if pack.manifest.excludes(target) {
            continue;
        }
        let Some(events) = pack.manifest.hooks.get(&target) else {
            continue;
        };
        let scripts = script_plan(config, pack, target)?;
        let rewrites = scripts
            .into_iter()
            .map(|(source, destination, _)| (source, destination))
            .collect::<Vec<_>>();
        for (event, entries) in events {
            let output = managed.entry(event.clone()).or_default();
            for (ordinal, entry) in entries.iter().enumerate() {
                let mut entry = entry.clone();
                rewrite_commands(&mut entry, &rewrites);
                entry.insert(
                    "_as".to_owned(),
                    Value::String(format!(
                        "agent-sync:{}:{}:{event}:{ordinal}",
                        pack.name, pack.manifest.version
                    )),
                );
                output.push(Value::Object(entry));
            }
        }
    }
    Ok(managed)
}

fn managed_entries(root: &Value) -> Result<BTreeMap<String, Vec<Value>>> {
    let mut managed = BTreeMap::new();
    let Some(hooks) = root.get("hooks") else {
        return Ok(managed);
    };
    for (event, entries) in hooks
        .as_object()
        .context("hook config 'hooks' must be a JSON object")?
    {
        let entries = entries
            .as_array()
            .context("hook event value must be an array")?
            .iter()
            .filter(|entry| {
                entry
                    .get("_as")
                    .and_then(Value::as_str)
                    .is_some_and(|tag| tag.starts_with(MANAGED_PREFIX))
            })
            .cloned()
            .collect::<Vec<_>>();
        if !entries.is_empty() {
            managed.insert(event.clone(), entries);
        }
    }
    Ok(managed)
}

fn read_config(path: &Path) -> Result<Value> {
    if !path.exists() {
        return Ok(Value::Object(Map::new()));
    }
    let text = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("parse {}", path.display()))
}

fn write_json_atomic(path: &Path, value: &Value) -> Result<()> {
    let parent = path.parent().context("hook config has no parent")?;
    fs::create_dir_all(parent)
        .with_context(|| format!("create hook config parent {}", parent.display()))?;
    let temporary = parent.join(format!(".hooks.{}.tmp", std::process::id()));
    let mut text = serde_json::to_string_pretty(value)?;
    text.push('\n');
    fs::write(&temporary, text)
        .with_context(|| format!("write temporary config {}", temporary.display()))?;
    fs::rename(&temporary, path)
        .with_context(|| format!("replace hook config {}", path.display()))?;
    Ok(())
}
