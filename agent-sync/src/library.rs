use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{Context, Result};

use crate::config::Config;
use crate::manifest::Manifest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Skills,
    Commands,
    Agents,
    Hooks,
}

impl Kind {
    pub const ALL: [Self; 4] = [Self::Skills, Self::Commands, Self::Agents, Self::Hooks];

    #[must_use]
    pub const fn dir_name(self) -> &'static str {
        match self {
            Self::Skills => "skills",
            Self::Commands => "commands",
            Self::Agents => "agents",
            Self::Hooks => "hooks",
        }
    }

    #[must_use]
    pub const fn source_file(self) -> Option<&'static str> {
        match self {
            Self::Skills => Some("SKILL.md"),
            Self::Commands => Some("COMMAND.md"),
            Self::Agents => Some("AGENT.md"),
            Self::Hooks => None,
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.dir_name())
    }
}

/// Where sync resolves Library items from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, clap::ValueEnum)]
pub enum SourceMode {
    /// Local + public libraries only (historical default).
    #[default]
    Library,
    /// Harness pull cache only.
    Cache,
    /// local_library > cache_library > public_library; tombstones still win.
    Hybrid,
}

impl fmt::Display for SourceMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Library => formatter.write_str("library"),
            Self::Cache => formatter.write_str("cache"),
            Self::Hybrid => formatter.write_str("hybrid"),
        }
    }
}

impl FromStr for SourceMode {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "library" => Ok(Self::Library),
            "cache" => Ok(Self::Cache),
            "hybrid" => Ok(Self::Hybrid),
            other => Err(format!(
                "unknown source '{other}'; expected library, cache, or hybrid"
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemSource {
    Public,
    Local,
    Cache,
}

impl fmt::Display for ItemSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Public => formatter.write_str("public"),
            Self::Local => formatter.write_str("local"),
            Self::Cache => formatter.write_str("cache"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LibraryItem {
    pub kind: Kind,
    pub name: String,
    pub vendor_origin: Option<String>,
    pub path: PathBuf,
    pub source: ItemSource,
    pub shadows_public: bool,
    pub manifest: Manifest,
}

impl LibraryItem {
    #[must_use]
    pub fn source_file(&self) -> Option<PathBuf> {
        self.kind.source_file().map(|name| self.path.join(name))
    }

    #[must_use]
    pub fn display_name(&self) -> String {
        self.vendor_origin.as_ref().map_or_else(
            || self.name.clone(),
            |origin| format!("vendor/{origin}/{}", self.name),
        )
    }
}

#[derive(Debug, Clone)]
pub struct LibraryDiagnostic {
    pub message: String,
}

#[derive(Debug, Default)]
pub struct Library {
    pub items: Vec<LibraryItem>,
    pub tombstones: Vec<String>,
    pub diagnostics: Vec<LibraryDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ItemKey {
    kind: Kind,
    vendor_origin: Option<String>,
    name: String,
}

#[derive(Debug)]
enum Candidate {
    Item(LibraryItem),
    Tombstone(PathBuf),
}

impl Library {
    /// Scan with the historical library-only source (local > public).
    pub fn scan(config: &Config) -> Result<Self> {
        Self::scan_with_source(config, SourceMode::Library)
    }

    /// Scan using the requested source mode.
    pub fn scan_with_source(config: &Config, mode: SourceMode) -> Result<Self> {
        let layers: Vec<(PathBuf, ItemSource)> = match mode {
            SourceMode::Library => {
                if config.local_library == config.public_library {
                    vec![(config.public_library.clone(), ItemSource::Public)]
                } else {
                    vec![
                        (config.local_library.clone(), ItemSource::Local),
                        (config.public_library.clone(), ItemSource::Public),
                    ]
                }
            }
            SourceMode::Cache => vec![(config.cache_library(), ItemSource::Cache)],
            SourceMode::Hybrid => {
                let mut layers = Vec::new();
                if config.local_library != config.public_library {
                    layers.push((config.local_library.clone(), ItemSource::Local));
                }
                layers.push((config.cache_library(), ItemSource::Cache));
                layers.push((config.public_library.clone(), ItemSource::Public));
                layers
            }
        };
        merge_layers(&layers)
    }
}

fn merge_layers(layers: &[(PathBuf, ItemSource)]) -> Result<Library> {
    let scanned: Vec<(ItemSource, BTreeMap<ItemKey, Candidate>)> = layers
        .iter()
        .map(|(root, source)| Ok((*source, scan_root(root, *source)?)))
        .collect::<Result<_>>()?;

    let mut claimed = BTreeSet::new();
    let mut result = Library::default();

    for (index, (source, map)) in scanned.iter().enumerate() {
        for (key, candidate) in map {
            if claimed.contains(key) {
                continue;
            }
            claimed.insert(key.clone());
            match candidate {
                Candidate::Tombstone(path) => {
                    let label = key_label(key);
                    result.tombstones.push(label.clone());
                    let lower_has_item = scanned[index + 1..]
                        .iter()
                        .any(|(_, lower)| matches!(lower.get(key), Some(Candidate::Item(_))));
                    if !lower_has_item {
                        result.diagnostics.push(LibraryDiagnostic {
                            message: format!("orphan tombstone {label} at {}", path.display()),
                        });
                    }
                }
                Candidate::Item(item) => {
                    let mut item = item.clone();
                    let lower_has_item = scanned[index + 1..]
                        .iter()
                        .any(|(_, lower)| matches!(lower.get(key), Some(Candidate::Item(_))));
                    item.shadows_public = lower_has_item;
                    if *source == ItemSource::Local && !item.shadows_public {
                        result.diagnostics.push(LibraryDiagnostic {
                            message: format!("local orphan override {}", key_label(key)),
                        });
                    } else if *source == ItemSource::Local
                        && item.shadows_public
                        && key.vendor_origin.is_some()
                    {
                        result.diagnostics.push(LibraryDiagnostic {
                            message: format!("local vendor shadow {}", key_label(key)),
                        });
                    }
                    result.items.push(item);
                }
            }
        }
    }

    result.items.sort_by(|left, right| {
        (left.kind, &left.vendor_origin, &left.name).cmp(&(
            right.kind,
            &right.vendor_origin,
            &right.name,
        ))
    });
    result.tombstones.sort();
    Ok(result)
}

fn scan_root(root: &Path, source: ItemSource) -> Result<BTreeMap<ItemKey, Candidate>> {
    let mut found = BTreeMap::new();
    if !root.is_dir() {
        return Ok(found);
    }
    for kind in Kind::ALL {
        let kind_root = root.join(kind.dir_name());
        if !kind_root.is_dir() {
            continue;
        }
        for entry in sorted_entries(&kind_root)? {
            let name = entry.file_name().to_string_lossy().into_owned();
            let path = entry.path();
            if kind == Kind::Skills && name == "vendor" && path.is_dir() {
                scan_vendor(&path, source, &mut found)?;
                continue;
            }
            if !path.is_dir() {
                continue;
            }
            insert_candidate(&mut found, kind, name, None, path, source)?;
        }
    }
    Ok(found)
}

fn scan_vendor(
    vendor_root: &Path,
    source: ItemSource,
    found: &mut BTreeMap<ItemKey, Candidate>,
) -> Result<()> {
    for origin_entry in sorted_entries(vendor_root)? {
        let origin_path = origin_entry.path();
        if !origin_path.is_dir() {
            continue;
        }
        let origin = origin_entry.file_name().to_string_lossy().into_owned();
        for item_entry in sorted_entries(&origin_path)? {
            let item_path = item_entry.path();
            if !item_path.is_dir() {
                continue;
            }
            insert_candidate(
                found,
                Kind::Skills,
                item_entry.file_name().to_string_lossy().into_owned(),
                Some(origin.clone()),
                item_path,
                source,
            )?;
        }
    }
    Ok(())
}

fn insert_candidate(
    found: &mut BTreeMap<ItemKey, Candidate>,
    kind: Kind,
    name: String,
    vendor_origin: Option<String>,
    path: PathBuf,
    source: ItemSource,
) -> Result<()> {
    let key = ItemKey {
        kind,
        vendor_origin: vendor_origin.clone(),
        name: name.clone(),
    };
    if path.join(".agent-sync-tombstone").is_file() {
        found.insert(key, Candidate::Tombstone(path));
        return Ok(());
    }
    let valid = kind.source_file().map_or_else(
        || path.join("manifest.toml").is_file(),
        |file| path.join(file).is_file(),
    );
    if !valid {
        return Ok(());
    }
    let manifest = Manifest::load(&path)?;
    found.insert(
        key,
        Candidate::Item(LibraryItem {
            kind,
            name,
            vendor_origin,
            path,
            source,
            shadows_public: false,
            manifest,
        }),
    );
    Ok(())
}

fn sorted_entries(path: &Path) -> Result<Vec<fs::DirEntry>> {
    let mut entries = fs::read_dir(path)
        .with_context(|| format!("read Library directory {}", path.display()))?
        .collect::<std::io::Result<Vec<_>>>()
        .with_context(|| format!("read entries under {}", path.display()))?;
    entries.sort_by_key(fs::DirEntry::file_name);
    Ok(entries)
}

fn key_label(key: &ItemKey) -> String {
    key.vendor_origin.as_ref().map_or_else(
        || format!("{}/{}", key.kind, key.name),
        |origin| format!("{}/vendor/{origin}/{}", key.kind, key.name),
    )
}
