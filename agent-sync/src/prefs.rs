//! Local harness preferences at `~/.agent-sync/preferences.json`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Staff preferences that gate live pull and filter the harness catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Preferences {
    pub live_sync: LiveSync,
    /// itemId (or name) → contentHash; pin forces that hash even if tip moved.
    #[serde(default)]
    pub pins: BTreeMap<String, String>,
    /// itemIds (or names) suppressed from pull/fan-out.
    #[serde(default)]
    pub tombstones: Vec<String>,
    #[serde(default = "default_targets_enabled")]
    pub targets_enabled: BTreeMap<String, bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LiveSync {
    /// `stale_check` (default) or `off`.
    pub mode: String,
    pub channel: String,
}

fn default_targets_enabled() -> BTreeMap<String, bool> {
    BTreeMap::from([
        ("claude".to_owned(), true),
        ("cursor".to_owned(), true),
        ("pi".to_owned(), true),
    ])
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            live_sync: LiveSync {
                mode: "stale_check".to_owned(),
                channel: "stable".to_owned(),
            },
            pins: BTreeMap::new(),
            tombstones: Vec::new(),
            targets_enabled: default_targets_enabled(),
        }
    }
}

impl Preferences {
    /// True when sessionStart / `--if-stale` pulls should be skipped.
    #[must_use]
    pub fn live_sync_off(&self) -> bool {
        self.live_sync.mode.eq_ignore_ascii_case("off")
    }

    /// Whether a Target id (`claude` / `cursor` / `pi`) is enabled for hybrid sync.
    #[must_use]
    pub fn target_enabled(&self, target_id: &str) -> bool {
        self.targets_enabled
            .get(target_id)
            .copied()
            .unwrap_or(true)
    }

    /// True when `id_or_name` is tombstoned.
    #[must_use]
    pub fn is_tombstoned(&self, id_or_name: &str) -> bool {
        self.tombstones.iter().any(|t| t == id_or_name)
    }

    /// Resolve pin for item_id, falling back to name.
    #[must_use]
    pub fn pin_for(&self, item_id: &str, name: &str) -> Option<&str> {
        self.pins
            .get(item_id)
            .or_else(|| self.pins.get(name))
            .map(String::as_str)
    }
}

/// `~/.agent-sync/preferences.json`
#[must_use]
pub fn preferences_path(home: &Path) -> PathBuf {
    home.join(".agent-sync/preferences.json")
}

/// Load preferences, or defaults when the file is missing.
pub fn load(home: &Path) -> Result<Preferences> {
    let path = preferences_path(home);
    if !path.exists() {
        return Ok(Preferences::default());
    }
    let raw = fs::read_to_string(&path)
        .with_context(|| format!("read preferences at {}", path.display()))?;
    let prefs: Preferences = serde_json::from_str(&raw)
        .with_context(|| format!("parse preferences at {}", path.display()))?;
    Ok(prefs)
}

/// Write preferences (creates `~/.agent-sync/` as needed).
pub fn save(home: &Path, prefs: &Preferences) -> Result<()> {
    let path = preferences_path(home);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create preferences parent {}", parent.display()))?;
    }
    let body = serde_json::to_string_pretty(prefs).context("serialize preferences")?;
    fs::write(&path, format!("{body}\n"))
        .with_context(|| format!("write preferences at {}", path.display()))?;
    Ok(())
}

/// Ensure a preferences file exists (writes defaults when missing).
pub fn ensure_defaults(home: &Path) -> Result<Preferences> {
    let path = preferences_path(home);
    if path.exists() {
        return load(home);
    }
    let prefs = Preferences::default();
    save(home, &prefs)?;
    Ok(prefs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn default_round_trip_json() {
        let prefs = Preferences::default();
        let json = serde_json::to_string_pretty(&prefs).unwrap();
        let parsed: Preferences = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.live_sync.mode, "stale_check");
        assert_eq!(parsed.live_sync.channel, "stable");
        assert!(parsed.target_enabled("pi"));
    }

    #[test]
    fn ensure_defaults_writes_file() {
        let dir = tempdir().unwrap();
        let prefs = ensure_defaults(dir.path()).unwrap();
        assert!(!prefs.live_sync_off());
        assert!(preferences_path(dir.path()).is_file());
        let again = load(dir.path()).unwrap();
        assert_eq!(again, prefs);
    }

    #[test]
    fn tombstone_and_pin_helpers() {
        let mut prefs = Preferences::default();
        prefs.tombstones.push("dead-id".to_owned());
        prefs
            .pins
            .insert("keep-id".to_owned(), "pinned-hash".to_owned());
        assert!(prefs.is_tombstoned("dead-id"));
        assert_eq!(prefs.pin_for("keep-id", "kept"), Some("pinned-hash"));
    }
}
