use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::library::Kind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Target {
    Claude,
    Cursor,
    Opencode,
    Pi,
}

impl Target {
    pub const ALL: [Self; 4] = [Self::Claude, Self::Cursor, Self::Opencode, Self::Pi];

    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Claude => "claude",
            Self::Cursor => "cursor",
            Self::Opencode => "opencode",
            Self::Pi => "pi",
        }
    }

    #[must_use]
    pub const fn supports(self, kind: Kind) -> bool {
        match self {
            Self::Claude | Self::Cursor | Self::Pi => true,
            Self::Opencode => !matches!(kind, Kind::Hooks),
        }
    }

    #[must_use]
    pub const fn unsupported_reason(self, kind: Kind) -> Option<&'static str> {
        if self.supports(kind) {
            return None;
        }

        Some(match (self, kind) {
            (Self::Opencode, Kind::Hooks) => "native hooks are unsupported",
            _ => "kind is unsupported",
        })
    }

    #[must_use]
    pub fn destination(self, home: &Path, kind: Kind, fanout_name: &str) -> Option<PathBuf> {
        let path = match (self, kind) {
            (Self::Claude, Kind::Skills) => home.join(".claude/skills").join(fanout_name),
            (Self::Claude, Kind::Commands) => home
                .join(".claude/commands")
                .join(format!("{fanout_name}.md")),
            (Self::Claude, Kind::Agents) => home
                .join(".claude/agents")
                .join(format!("{fanout_name}.md")),
            (Self::Cursor, Kind::Skills | Kind::Commands) => {
                home.join(".cursor/skills").join(fanout_name)
            }
            (Self::Cursor, Kind::Agents) => home
                .join(".cursor/agents")
                .join(format!("{fanout_name}.md")),
            (Self::Opencode, Kind::Skills) => {
                home.join(".config/opencode/skills").join(fanout_name)
            }
            (Self::Opencode, Kind::Commands) => home
                .join(".config/opencode/commands")
                .join(format!("{fanout_name}.md")),
            (Self::Opencode, Kind::Agents) => home
                .join(".config/opencode/agents")
                .join(format!("{fanout_name}.md")),
            // Commands map to skill packages: Pi has no native commands dir.
            (Self::Pi, Kind::Skills | Kind::Commands) => {
                home.join(".pi/agent/skills").join(fanout_name)
            }
            (Self::Pi, Kind::Agents) => home
                .join(".pi/agent/agents")
                .join(format!("{fanout_name}.md")),
            _ => return None,
        };
        Some(path)
    }

    #[must_use]
    pub fn hooks_config(self, home: &Path) -> Option<PathBuf> {
        match self {
            Self::Claude => Some(home.join(".claude/settings.json")),
            Self::Cursor => Some(home.join(".cursor/hooks.json")),
            // Pi uses TypeScript extensions + a managed registry file (no JSON merge).
            Self::Opencode | Self::Pi => None,
        }
    }

    #[must_use]
    pub fn hooks_dir(self, home: &Path) -> Option<PathBuf> {
        match self {
            Self::Claude => Some(home.join(".claude/hooks")),
            Self::Cursor => Some(home.join(".cursor/hooks")),
            // Spike (Pi 0.84): hooks/ was renamed to extensions/; Pi warns if hooks/ exists.
            Self::Pi => Some(home.join(".pi/agent/extensions")),
            Self::Opencode => None,
        }
    }

    /// Registry file for targets that do not merge into a host JSON settings file.
    #[must_use]
    pub fn hooks_managed_registry(self, home: &Path) -> Option<PathBuf> {
        match self {
            Self::Pi => Some(
                home.join(".pi/agent/extensions")
                    .join(".agent-sync-managed.json"),
            ),
            _ => None,
        }
    }
}

impl fmt::Display for Target {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.id())
    }
}

impl FromStr for Target {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "claude" => Ok(Self::Claude),
            "cursor" => Ok(Self::Cursor),
            "opencode" => Ok(Self::Opencode),
            "pi" => Ok(Self::Pi),
            "agents" => bail!("target 'agents' is reserved and not supported in v1"),
            other => bail!("unknown target '{other}'"),
        }
    }
}
