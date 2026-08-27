use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::config::Config;
use crate::frontmatter;
use crate::hooks;
use crate::install::{self, InstallMode, InstallOutcome};
use crate::library::{Kind, Library};
use crate::overlay;
use crate::state::{InstalledPath, State};
use crate::target::Target;

#[derive(Debug, Clone)]
pub struct ExpectedMarkdown {
    pub kind: Kind,
    pub item_name: String,
    pub source_root: PathBuf,
    pub target: Target,
    pub destination: PathBuf,
    pub main_destination: PathBuf,
    pub content: String,
    pub excluded: bool,
}

pub fn run(config: &Config, dry_run: bool) -> Result<()> {
    let library = Library::scan(config)?;
    for diagnostic in &library.diagnostics {
        eprintln!("WARN {}", diagnostic.message);
    }
    for tombstone in &library.tombstones {
        println!("SKIP {tombstone} (local tombstone)");
    }

    let old_state = State::load(&config.state_file)?;
    let expected_items = expected_markdown(config, &library)?;
    if !dry_run {
        install::remove_path(&config.wrapper_root)?;
        fs::create_dir_all(&config.wrapper_root)
            .with_context(|| format!("create wrapper root {}", config.wrapper_root.display()))?;
    }

    let mut new_entries = Vec::new();
    let mut desired_paths = BTreeSet::new();
    for expected in expected_items {
        if expected.excluded {
            println!(
                "SKIP {}/{} -> {} (excluded)",
                expected.kind, expected.item_name, expected.target
            );
            continue;
        }
        let wrapper_source = wrapper_source(config, &expected);
        if !dry_run {
            materialize_wrapper(&expected, &wrapper_source)?;
        }
        desired_paths.insert(expected.destination.clone());
        let preferred = if expected.target == Target::Cursor {
            InstallMode::Copy
        } else {
            InstallMode::Symlink
        };
        match install::install(
            &wrapper_source,
            &expected.destination,
            preferred,
            old_state.owns(&expected.destination),
            dry_run,
        )? {
            InstallOutcome::Installed(mode) => {
                println!(
                    "{} {}/{} -> {}:{} ({})",
                    if dry_run { "PLAN" } else { "SYNC" },
                    expected.kind,
                    expected.item_name,
                    expected.target,
                    expected.destination.display(),
                    mode.as_str()
                );
                new_entries.push(InstalledPath::new(
                    expected.source_root,
                    expected.target.id(),
                    expected.destination,
                    mode.as_str(),
                    expected.kind.to_string(),
                    expected.item_name,
                ));
            }
            InstallOutcome::SkippedForeign(reason) => {
                eprintln!(
                    "WARN {}/{} -> {} skipped: {reason}",
                    expected.kind, expected.item_name, expected.target
                );
                if let Some(previous) = old_state
                    .fanouts
                    .iter()
                    .find(|entry| entry.agent_path == expected.destination)
                {
                    new_entries.push(previous.clone());
                }
            }
        }
    }

    print_unsupported(&library);

    let hook_packs = library
        .items
        .iter()
        .filter(|item| item.kind == Kind::Hooks)
        .collect::<Vec<_>>();
    let hook_entries = hooks::sync(config, &hook_packs, &old_state, dry_run)?;
    desired_paths.extend(hook_entries.iter().map(|entry| entry.agent_path.clone()));
    new_entries.extend(hook_entries);

    reconcile_stale(&old_state, &desired_paths, &mut new_entries, dry_run)?;

    if !dry_run {
        new_entries.sort_by(|left, right| left.agent_path.cmp(&right.agent_path));
        new_entries.dedup_by(|left, right| left.agent_path == right.agent_path);
        State {
            version: 1,
            fanouts: new_entries,
        }
        .write(&config.state_file)?;
        println!("STATE {}", config.state_file.display());
    }
    Ok(())
}

fn print_unsupported(library: &Library) {
    for item in &library.items {
        for target in Target::ALL {
            if !target.supports(item.kind) {
                println!(
                    "INFO {}/{} -> {target} skipped: {}",
                    item.kind,
                    item.display_name(),
                    target
                        .unsupported_reason(item.kind)
                        .unwrap_or("unsupported")
                );
            }
        }
    }
}

pub fn expected_markdown(config: &Config, library: &Library) -> Result<Vec<ExpectedMarkdown>> {
    // helper stays in-module

    let skill_names = library
        .items
        .iter()
        .filter(|item| item.kind == Kind::Skills)
        .map(|item| item.name.clone())
        .collect::<BTreeSet<_>>();
    let mut expected = Vec::new();
    for item in &library.items {
        if item.kind == Kind::Hooks {
            continue;
        }
        let source_file = item
            .source_file()
            .context("markdown item must have a source file")?;
        let source = fs::read_to_string(&source_file)
            .with_context(|| format!("read {}", source_file.display()))?;
        let fanout_name = config.fanout_name(&item.name, item.vendor_origin.as_deref());
        for target in Target::ALL {
            if !target.supports(item.kind) {
                continue;
            }
            // Cursor installs commands into the skills tree; skip when a skill
            // already owns that fan-out name so verify/sync do not thrash.
            let cursor_command_shadowed = item.kind == Kind::Commands
                && target == Target::Cursor
                && skill_names.contains(&item.name);
            let destination = target
                .destination(&config.target_home, item.kind, &fanout_name)
                .context("supported target must have a destination")?;
            let main_destination = destination_main(&destination, item.kind, target);

            let mut content = overlay::apply(&source, &item.manifest.overlay(target))?;

            // Pi agents should not carry model invocation aliases.
            if item.kind == Kind::Agents && target == Target::Pi {
                content = remove_pi_agent_model_alias(&content)?;
            }

            expected.push(ExpectedMarkdown {
                kind: item.kind,
                item_name: item.display_name(),
                source_root: item.path.clone(),
                target,
                destination,
                main_destination,
                content,
                excluded: item.manifest.excludes(target) || cursor_command_shadowed,
            });
        }
    }
    Ok(expected)
}

fn destination_main(destination: &Path, kind: Kind, target: Target) -> PathBuf {
    if kind == Kind::Skills || (kind == Kind::Commands && target == Target::Cursor) {
        destination.join("SKILL.md")
    } else {
        destination.to_path_buf()
    }
}

fn remove_pi_agent_model_alias(content: &str) -> Result<String> {
    let mut md = frontmatter::parse(content)?;
    if let Some(map) = md.frontmatter.as_mapping_mut() {
        map.remove(&serde_yaml::Value::String("model".to_owned()));
    }
    frontmatter::render(&md)
}

fn wrapper_source(config: &Config, expected: &ExpectedMarkdown) -> PathBuf {
    let name = expected
        .destination
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    config
        .wrapper_root
        .join(expected.target.id())
        .join(expected.kind.dir_name())
        .join(name.as_ref())
}

fn materialize_wrapper(expected: &ExpectedMarkdown, wrapper: &Path) -> Result<()> {
    install::remove_path(wrapper)?;
    let directory_install = expected.kind == Kind::Skills
        || (expected.kind == Kind::Commands && expected.target == Target::Cursor);
    if directory_install {
        install::copy_path(&expected.source_root, wrapper)?;
        install::remove_path(&wrapper.join("manifest.toml"))?;
        if let Some(source_name) = expected.kind.source_file() {
            if source_name != "SKILL.md" {
                install::remove_path(&wrapper.join(source_name))?;
            }
        }
        fs::write(wrapper.join("SKILL.md"), &expected.content)
            .with_context(|| format!("write wrapper {}", wrapper.display()))?;
    } else {
        let parent = wrapper.parent().context("wrapper file has no parent")?;
        fs::create_dir_all(parent)
            .with_context(|| format!("create wrapper parent {}", parent.display()))?;
        fs::write(wrapper, &expected.content)
            .with_context(|| format!("write wrapper {}", wrapper.display()))?;
    }
    Ok(())
}

fn reconcile_stale(
    old_state: &State,
    desired_paths: &BTreeSet<PathBuf>,
    new_entries: &mut Vec<InstalledPath>,
    dry_run: bool,
) -> Result<()> {
    for stale in old_state
        .fanouts
        .iter()
        .filter(|entry| !desired_paths.contains(&entry.agent_path))
    {
        if install::is_npx_owned_symlink(&stale.agent_path)? {
            eprintln!(
                "WARN stale state now points into .agents/skills; preserving {}",
                stale.agent_path.display()
            );
            continue;
        }
        println!(
            "{} stale {}",
            if dry_run { "PLAN remove" } else { "REMOVE" },
            stale.agent_path.display()
        );
        if !dry_run {
            install::remove_path(&stale.agent_path)?;
        }
    }
    new_entries.retain(|entry| desired_paths.contains(&entry.agent_path));
    Ok(())
}
