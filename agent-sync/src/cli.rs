use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::library::SourceMode;

#[derive(Debug, Parser)]
#[command(name = "agent-sync", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Fan out the effective Library to every supported Target.
    Sync(SyncArgs),
    /// Check installed Target files against the effective Library.
    Verify(RootArgs),
    /// Show effective, shadowed, and tombstoned Library items.
    List(RootArgs),
    /// Move legacy per-Target trees into the Library.
    Migrate(MigrateArgs),
    /// Detect (and optionally fix) legacy home→repo Target symlinks.
    Doctor(DoctorArgs),
    /// Publish allowlisted platform-repo `.claude` items to harness `latest`.
    Publish(PublishArgs),
    /// Pull harness channel revisions into `~/.agent-sync/cache/<channel>/`.
    Pull(PullArgs),
}

#[derive(Debug, Args)]
pub struct SyncArgs {
    /// Print the fan-out plan without writing anything.
    #[arg(long)]
    pub dry_run: bool,

    /// Library resolution: `library` (default), `cache`, or `hybrid`.
    #[arg(long, value_enum, default_value_t = SourceMode::Library)]
    pub source: SourceMode,

    /// Override the Target home base (for sandboxes and tests).
    #[arg(long, value_name = "SANDBOX")]
    pub root: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct PullArgs {
    /// Harness channel to pull (`stable` or `latest`).
    #[arg(long, default_value = "stable")]
    pub channel: String,

    /// Send If-None-Match from the cached ETAG; keep cache on 304.
    #[arg(long)]
    pub if_stale: bool,

    /// harness API base URL (default: AGENT_SYNC_HARNESS_URL or http://localhost:8081).
    #[arg(long, value_name = "URL", env = "AGENT_SYNC_HARNESS_URL")]
    pub base_url: Option<String>,
}

#[derive(Debug, Args)]
pub struct RootArgs {
    /// Override the Target home base (for sandboxes and tests).
    #[arg(long, value_name = "SANDBOX")]
    pub root: Option<PathBuf>,
}

#[derive(Debug, Args)]
#[allow(clippy::struct_excessive_bools)]
pub struct MigrateArgs {
    /// Print the migration plan without mutating files.
    #[arg(long, conflicts_with_all = ["write", "rollback"])]
    pub dry_run: bool,

    /// Apply the migration, back it up, fan out, and verify.
    #[arg(long, conflicts_with_all = ["dry_run", "rollback"])]
    pub write: bool,

    /// Include supported Target directories in the migration backup.
    #[arg(long, requires = "write")]
    pub backup_targets: bool,

    /// Permit tracked or untracked changes under legacy Target trees.
    #[arg(long, requires = "write")]
    pub allow_dirty: bool,

    /// Use disposition rows from a Markdown inventory.
    #[arg(long, value_name = "PATH")]
    pub inventory: Option<PathBuf>,

    /// Restore a timestamped migration backup.
    #[arg(long, value_name = "ID", conflicts_with_all = ["dry_run", "write"])]
    pub rollback: Option<String>,

    /// Override the Target home base (for sandboxes and tests).
    #[arg(long, value_name = "SANDBOX")]
    pub root: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct DoctorArgs {
    /// Replace legacy home→repo symlinks with empty real directories.
    #[arg(long)]
    pub fix: bool,

    /// Override the Target home base (for sandboxes and tests).
    #[arg(long, value_name = "SANDBOX")]
    pub root: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct PublishArgs {
    /// Comma-separated pilot skill/command names (must be in the closed allowlist).
    #[arg(long, value_name = "NAMES")]
    pub items: String,

    /// Path to platform repo root (contains `.claude`) (contains `.claude/skills|commands`).
    #[arg(long, value_name = "PATH")]
    pub platform_root: PathBuf,

    /// Print the publish envelope without POSTing.
    #[arg(long)]
    pub dry_run: bool,

    /// harness API base URL (default: AGENT_SYNC_HARNESS_URL or http://localhost:8081).
    #[arg(long, value_name = "URL", env = "AGENT_SYNC_HARNESS_URL")]
    pub base_url: Option<String>,
}
