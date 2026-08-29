use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::config::Config;
use crate::http;

const MANAGED_RELS: &[&str] = &[
    ".claude/skills",
    ".claude/commands",
    ".claude/agents",
    ".cursor/skills",
    ".cursor/commands",
    ".cursor/agents",
];

/// Product tree that may stay symlinked into the clone.
const PRODUCT_RELS: &[&str] = &[".cursor/skills-cursor"];

#[derive(Debug)]
struct LinkIssue {
    path: PathBuf,
    target: PathBuf,
}

pub fn run(config: &Config, fix: bool) -> Result<()> {
    warn_harness_token_if_needed();

    let mut issues = Vec::new();
    for relative in MANAGED_RELS {
        let path = config.target_home.join(relative);
        if let Some(issue) = legacy_repo_symlink(config, &path)? {
            issues.push(issue);
        }
    }

    for relative in PRODUCT_RELS {
        let path = config.target_home.join(relative);
        if path.is_symlink() {
            let target = fs::read_link(&path)
                .with_context(|| format!("read link {}", path.display()))?;
            let resolved = resolve_link(&path, &target);
            if resolved.starts_with(&config.dotfiles_dir) {
                println!(
                    "OK {} -> {} (product tree may stay in clone)",
                    path.display(),
                    target.display()
                );
            }
        }
    }

    if issues.is_empty() {
        println!("OK doctor: no legacy home→repo symlinks on managed trees");
        return Ok(());
    }

    for issue in &issues {
        println!(
            "WARN {} -> {} (points into the dotfiles clone)",
            issue.path.display(),
            issue.target.display()
        );
    }

    if !fix {
        bail!(
            "doctor found {} legacy symlink(s); re-run with --fix to replace them with real directories",
            issues.len()
        );
    }

    for issue in issues {
        detach_symlink(&issue.path)?;
        println!("FIXED {} (now a real directory)", issue.path.display());
    }
    println!("OK doctor: detached legacy symlinks; run `agent-sync sync` next");
    Ok(())
}

/// Warn when harness URL is configured but no staff PAT is available.
fn warn_harness_token_if_needed() {
    if http::harness_url_set() && !http::token_present() {
        println!(
            "WARN AGENT_SYNC_HARNESS_URL is set but no token found \
             (set AGENT_SYNC_HARNESS_TOKEN or ~/.agent-sync/credentials)"
        );
    }
}

fn legacy_repo_symlink(config: &Config, path: &Path) -> Result<Option<LinkIssue>> {
    if !path.is_symlink() {
        return Ok(None);
    }
    let target =
        fs::read_link(path).with_context(|| format!("read link {}", path.display()))?;
    let resolved = resolve_link(path, &target);
    if resolved.starts_with(&config.dotfiles_dir) {
        Ok(Some(LinkIssue { path: path.to_path_buf(), target }))
    } else {
        Ok(None)
    }
}

fn resolve_link(path: &Path, target: &Path) -> PathBuf {
    if target.is_absolute() {
        target.to_path_buf()
    } else {
        path.parent()
            .map_or_else(|| target.to_path_buf(), |parent| parent.join(target))
    }
}

fn detach_symlink(path: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(path)
        .with_context(|| format!("stat {}", path.display()))?;
    if !metadata.file_type().is_symlink() {
        bail!("{} is not a symlink", path.display());
    }
    fs::remove_file(path).with_context(|| format!("remove symlink {}", path.display()))?;
    fs::create_dir_all(path).with_context(|| format!("create directory {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::symlink;
    use tempfile::tempdir;

    #[test]
    fn detaches_symlink_into_empty_dir() {
        let dir = tempdir().unwrap();
        let link = dir.path().join("skills");
        let target = dir.path().join("repo-skills");
        fs::create_dir_all(&target).unwrap();
        symlink(&target, &link).unwrap();
        detach_symlink(&link).unwrap();
        assert!(link.is_dir());
        assert!(!link.is_symlink());
    }
}
