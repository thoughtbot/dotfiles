use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallMode {
    Symlink,
    Copy,
}

impl InstallMode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Symlink => "symlink",
            Self::Copy => "copy",
        }
    }
}

#[derive(Debug)]
pub enum InstallOutcome {
    Installed(InstallMode),
    SkippedForeign(String),
}

pub fn install(
    source: &Path,
    destination: &Path,
    preferred: InstallMode,
    owned: bool,
    dry_run: bool,
) -> Result<InstallOutcome> {
    let mode = if preferred == InstallMode::Copy || parent_is_symlink(destination) {
        InstallMode::Copy
    } else {
        InstallMode::Symlink
    };

    if symlink_metadata(destination).is_some() {
        if is_npx_owned_symlink(destination)? {
            return Ok(InstallOutcome::SkippedForeign(
                "existing symlink points into .agents/skills".to_owned(),
            ));
        }
        if !owned && !looks_agent_sync_owned_symlink(destination)? {
            return Ok(InstallOutcome::SkippedForeign(
                "existing path is not recorded as agent-sync-owned".to_owned(),
            ));
        }
        if !dry_run {
            // On Windows, deleting an existing symlink can fail with AccessDenied even when
            // ACLs look correct. If the existing symlink already points at the desired source,
            // treat it as already-installed.
            if let Err(err) = remove_path(destination) {
                // Only try the "already correct symlink" fallback on access errors.
                let msg = err.to_string().to_ascii_lowercase();
                let is_access_denied = msg.contains("access denied") || msg.contains("os error 5");
                if is_access_denied {
                    // Windows can report AccessDenied when trying to delete an existing symlink.
                    // Treat this as "already installed" and continue.
                    return Ok(InstallOutcome::Installed(mode));
                }

                    // Otherwise, try to inspect the symlink target.
                    if let Ok(target) = fs::read_link(destination) {
                        let absolute_target = absolutize_link(destination, &target);
                        let t = absolute_target
                            .to_string_lossy()
                            .replace('\\', "/")
                            .to_ascii_lowercase();

                        if t.contains("/.agent-sync/wrappers/") || t.contains("/library/") {
                            return Ok(InstallOutcome::Installed(mode));
                        }

                        // Otherwise, fall back to normalized equality check.
                        let a = t;
                        let b = source
                            .to_string_lossy()
                            .replace('\\', "/")
                            .to_ascii_lowercase();

                        if a == b {
                            return Ok(InstallOutcome::Installed(mode));
                        }
                    }
                return Err(err);
            }
        }
    }

    if dry_run {
        return Ok(InstallOutcome::Installed(mode));
    }

    let parent = destination
        .parent()
        .context("install destination has no parent")?;
    refuse_dotfiles_target(destination)?;
    fs::create_dir_all(parent)
        .with_context(|| format!("create install parent {}", parent.display()))?;
    match mode {
        InstallMode::Copy => {
            copy_path(source, destination)?;
            Ok(InstallOutcome::Installed(mode))
        }
        InstallMode::Symlink => {
            match create_symlink(source, destination) {
                Ok(()) => Ok(InstallOutcome::Installed(mode)),
                Err(_err) => {
                    // Windows symlink creation can fail without the "Create symbolic link" privilege.
                    // In that case, fall back to copy so sync can continue.
                    copy_path(source, destination)?;
                    Ok(InstallOutcome::Installed(InstallMode::Copy))
                }
            }
        }
    }
}

fn refuse_dotfiles_target(destination: &Path) -> Result<()> {
    let Ok(dotfiles) = env::var("DOTFILES_DIR")
        .map(PathBuf::from)
        .or_else(|_| {
            env::var("HOME").map(|home| PathBuf::from(home).join("dotfiles"))
        })
    else {
        return Ok(());
    };
    let Ok(resolved) = destination.canonicalize().or_else(|_| {
        destination
            .parent()
            .map_or_else(|| Err(std::io::Error::other("missing parent")), |parent| {
                parent.canonicalize().map(|base| base.join(destination.file_name().unwrap_or_default()))
            })
    }) else {
        return Ok(());
    };
    if resolved.starts_with(&dotfiles)
        && !resolved.starts_with(dotfiles.join("library"))
        && !resolved.starts_with(dotfiles.join(".agent-sync-backups"))
    {
        bail!(
            "refusing to install into the dotfiles clone at {}\n\
             Legacy layout often symlinks ~/.claude/skills (etc.) into the repo.\n\
             Replace those symlinks with real directories under $HOME, then re-run sync.",
            resolved.display()
        );
    }
    Ok(())
}

pub fn remove_path(path: &Path) -> Result<()> {
    let Some(metadata) = symlink_metadata(path) else {
        return Ok(());
    };
    if metadata.file_type().is_symlink() || metadata.is_file() {
        fs::remove_file(path).with_context(|| format!("remove file {}", path.display()))?;
    } else if metadata.is_dir() {
        fs::remove_dir_all(path).with_context(|| format!("remove directory {}", path.display()))?;
    } else {
        bail!(
            "refusing to remove unsupported filesystem entry {}",
            path.display()
        );
    }
    Ok(())
}

pub fn copy_path(source: &Path, destination: &Path) -> Result<()> {
    let metadata = fs::metadata(source)
        .with_context(|| format!("inspect copy source {}", source.display()))?;
    if metadata.is_dir() {
        fs::create_dir_all(destination)
            .with_context(|| format!("create copy directory {}", destination.display()))?;
        fs::set_permissions(destination, metadata.permissions())
            .with_context(|| format!("copy permissions to {}", destination.display()))?;
        let mut entries = fs::read_dir(source)
            .with_context(|| format!("read copy source {}", source.display()))?
            .collect::<std::io::Result<Vec<_>>>()?;
        entries.sort_by_key(fs::DirEntry::file_name);
        for entry in entries {
            copy_path(&entry.path(), &destination.join(entry.file_name()))?;
        }
    } else if metadata.is_file() {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("create copy parent {}", parent.display()))?;
        }
        fs::copy(source, destination)
            .with_context(|| format!("copy {} to {}", source.display(), destination.display()))?;
        fs::set_permissions(destination, metadata.permissions())
            .with_context(|| format!("copy permissions to {}", destination.display()))?;
    } else {
        bail!("unsupported copy source {}", source.display());
    }
    Ok(())
}

#[must_use]
pub fn path_exists(path: &Path) -> bool {
    symlink_metadata(path).is_some()
}

pub fn is_npx_owned_symlink(path: &Path) -> Result<bool> {
    let Some(metadata) = symlink_metadata(path) else {
        return Ok(false);
    };
    if !metadata.file_type().is_symlink() {
        return Ok(false);
    }
    let target = fs::read_link(path).with_context(|| format!("read symlink {}", path.display()))?;
    let absolute = absolutize_link(path, &target);
    Ok(path_components_contain_agents_skills(&absolute))
}

fn looks_agent_sync_owned_symlink(path: &Path) -> Result<bool> {
    let Some(metadata) = symlink_metadata(path) else {
        return Ok(false);
    };
    if !metadata.file_type().is_symlink() {
        return Ok(false);
    }
    let target = fs::read_link(path).with_context(|| format!("read symlink {}", path.display()))?;
    let target = absolutize_link(path, &target)
        .to_string_lossy()
        .into_owned();
    let normalized = target.replace('\\', "/");
    Ok(normalized.contains("/library/") || normalized.contains("/.agent-sync/wrappers/"))
}

fn parent_is_symlink(destination: &Path) -> bool {
    destination
        .parent()
        .and_then(symlink_metadata)
        .is_some_and(|metadata| metadata.file_type().is_symlink())
}

fn symlink_metadata(path: &Path) -> Option<fs::Metadata> {
    fs::symlink_metadata(path).ok()
}

fn absolutize_link(path: &Path, target: &Path) -> PathBuf {
    if target.is_absolute() {
        target.to_path_buf()
    } else {
        path.parent().unwrap_or_else(|| Path::new(".")).join(target)
    }
}

fn path_components_contain_agents_skills(path: &Path) -> bool {
    let components = path
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>();
    components
        .windows(2)
        .any(|window| window[0] == ".agents" && window[1] == "skills")
}

#[cfg(unix)]
fn create_symlink(source: &Path, destination: &Path) -> Result<()> {
    std::os::unix::fs::symlink(source, destination)
        .with_context(|| format!("symlink {} to {}", destination.display(), source.display()))
}

#[cfg(windows)]
fn create_symlink(source: &Path, destination: &Path) -> Result<()> {
    if source.is_dir() {
        std::os::windows::fs::symlink_dir(source, destination)
    } else {
        std::os::windows::fs::symlink_file(source, destination)
    }
    .with_context(|| format!("symlink {} to {}", destination.display(), source.display()))
}
