use std::collections::BTreeSet;
use std::fs;

use anyhow::{Context, Result};

use crate::config::Config;
use crate::hooks;
use crate::install;
use crate::library::{Kind, Library};
use crate::state::State;
use crate::sync;
use crate::target::Target;

pub fn run(config: &Config) -> Result<bool> {
    let library = Library::scan(config)?;
    let state = State::load(&config.state_file)?;
    for diagnostic in &library.diagnostics {
        eprintln!("WARN {}", diagnostic.message);
    }
    let (mut valid, mut expected_paths) = verify_markdown(config, &library, &state)?;
    print_unsupported(&library);

    let packs = library
        .items
        .iter()
        .filter(|item| item.kind == Kind::Hooks)
        .collect::<Vec<_>>();
    if !hooks::verify(config, &packs)? {
        valid = false;
    }
    for path in hooks::expected_script_paths(config, &packs)? {
        expected_paths.insert(path.clone());
        if !state.owns(&path) {
            eprintln!(
                "ERROR {} is not recorded in {}",
                path.display(),
                config.state_file.display()
            );
            valid = false;
        }
    }

    valid &= verify_state(config, &library, &state, &expected_paths);

    if valid {
        println!("OK verify passed");
    }
    Ok(valid)
}

fn verify_markdown(
    config: &Config,
    library: &Library,
    state: &State,
) -> Result<(bool, BTreeSet<std::path::PathBuf>)> {
    let mut valid = true;
    let mut expected_paths = BTreeSet::new();
    for expected in sync::expected_markdown(config, library)? {
        if expected.excluded {
            println!(
                "SKIP {}/{} -> {} (Manifest exclude)",
                expected.kind, expected.item_name, expected.target
            );
            continue;
        }
        expected_paths.insert(expected.destination.clone());
        if !install::path_exists(&expected.destination) {
            eprintln!(
                "ERROR missing {}/{} -> {} at {}",
                expected.kind,
                expected.item_name,
                expected.target,
                expected.destination.display()
            );
            valid = false;
            continue;
        }
        if install::is_npx_owned_symlink(&expected.destination)? {
            eprintln!(
                "ERROR {}/{} -> {} is npx-owned at {}",
                expected.kind,
                expected.item_name,
                expected.target,
                expected.destination.display()
            );
            valid = false;
            continue;
        }
        if !state.owns(&expected.destination) {
            eprintln!(
                "ERROR {} is not recorded in {}",
                expected.destination.display(),
                config.state_file.display()
            );
            valid = false;
        }
        valid &= verify_content_and_mode(&expected)?;
    }
    Ok((valid, expected_paths))
}

fn verify_content_and_mode(expected: &sync::ExpectedMarkdown) -> Result<bool> {
    let installed = fs::read_to_string(&expected.main_destination).with_context(|| {
        format!(
            "read installed markdown {}",
            expected.main_destination.display()
        )
    })?;
    let mut valid = true;
    if installed != expected.content {
        eprintln!(
            "ERROR content mismatch {}/{} -> {} at {}",
            expected.kind,
            expected.item_name,
            expected.target,
            expected.main_destination.display()
        );
        valid = false;
    }
    let metadata = fs::symlink_metadata(&expected.destination)
        .with_context(|| format!("inspect {}", expected.destination.display()))?;

    // For Pi we prefer symlinks, but accept copies when symlink creation is
    // not permitted in the current environment (e.g. Windows privilege gaps).
    let installed_is_symlink = metadata.file_type().is_symlink();
    if expected.target == Target::Pi {
        println!(
            "OK {}/{} -> {} ({})",
            expected.kind,
            expected.item_name,
            expected.target,
            if installed_is_symlink { "symlink" } else { "copy" }
        );
        return Ok(true);
    }

    let parent_is_symlink = expected
        .destination
        .parent()
        .and_then(|parent| fs::symlink_metadata(parent).ok())
        .is_some_and(|parent| parent.file_type().is_symlink());
    let should_copy = expected.target == Target::Cursor || parent_is_symlink;

    if should_copy == installed_is_symlink {
        eprintln!(
            "ERROR wrong install mode for {}/{} -> {} at {}",
            expected.kind,
            expected.item_name,
            expected.target,
            expected.destination.display()
        );
        valid = false;
    } else {
        println!(
            "OK {}/{} -> {} ({})",
            expected.kind,
            expected.item_name,
            expected.target,
            if should_copy { "copy" } else { "symlink" }
        );
    }

    Ok(valid)
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

fn verify_state(
    config: &Config,
    library: &Library,
    state: &State,
    expected_paths: &BTreeSet<std::path::PathBuf>,
) -> bool {
    let mut valid = true;
    if !library.items.is_empty() && !config.state_file.exists() {
        eprintln!("ERROR missing state file {}", config.state_file.display());
        valid = false;
    }
    for entry in &state.fanouts {
        if !install::path_exists(&entry.agent_path) {
            eprintln!("ERROR state path missing {}", entry.agent_path.display());
            valid = false;
        }
        if !expected_paths.contains(&entry.agent_path) {
            eprintln!("ERROR stale state path {}", entry.agent_path.display());
            valid = false;
        }
    }
    valid
}
