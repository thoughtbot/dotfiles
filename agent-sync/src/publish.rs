//! Publish allowlisted platform-repo `.claude` skills/commands to harness.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use base64::Engine;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::http;

/// Closed pilot skill names under `.claude/skills/`.
pub const PILOT_SKILLS: &[&str] = &[
    "ship-feature",
    "spec-driven",
    "_shared",
    "de-adversarial-reviewer",
    "test-benchmark",
    "vibe-test",
    "pr-review",
];

/// Closed pilot command names under `.claude/commands/` (without `.md`).
pub const PILOT_COMMANDS: &[&str] = &[
    "plan-create",
    "plan-review",
    "plan-update",
    "spec-driven",
    "pr-review-local",
    "pr-review-update",
    "commit-push-pr",
];

const MAX_INLINE_BYTES: u64 = 32_768;

#[derive(Debug, Clone)]
pub struct PublishConfig {
    pub platform_root: PathBuf,
    pub items: Vec<String>,
    pub base_url: String,
    pub dry_run: bool,
}

/// Build a harness publish envelope from allowlisted platform `.claude` items.
pub fn build_envelope(platform_root: &Path, items: &[String]) -> Result<Value> {
    if items.is_empty() {
        bail!("--items must list at least one pilot skill or command name");
    }

    let allowlist = pilot_allowlist();
    for name in items {
        if !allowlist.contains(name.as_str()) {
            bail!(
                "item '{name}' is outside the closed pilot allowlist; \
                 publish only allowlisted platform-repo .claude skills/commands"
            );
        }
    }

    let mut published = Vec::new();
    for name in items {
        let mut found = false;
        // Names may appear in both pilot skill and command sets (e.g. spec-driven).
        if PILOT_SKILLS.contains(&name.as_str()) {
            let skill_dir = platform_root.join(".claude/skills").join(name);
            if skill_dir.is_dir() {
                published.push(scan_skill(&skill_dir, name)?);
                found = true;
            }
        }
        if PILOT_COMMANDS.contains(&name.as_str()) {
            let command_path = platform_root
                .join(".claude/commands")
                .join(format!("{name}.md"));
            if command_path.is_file() {
                published.push(scan_command(&command_path, name)?);
                found = true;
            }
        }
        if !found {
            bail!(
                "pilot item '{name}' not found under {}/.claude/skills|commands",
                platform_root.display()
            );
        }
    }

    Ok(json!({
        "idempotencyKey": uuid::Uuid::new_v4().to_string(),
        "items": published,
    }))
}

/// Scan allowlisted items and POST to `/api/harness/publish` (or dry-run print).
pub fn publish(cfg: &PublishConfig) -> Result<()> {
    let envelope = build_envelope(&cfg.platform_root, &cfg.items)?;

    if cfg.dry_run {
        println!("{}", serde_json::to_string_pretty(&envelope)?);
        return Ok(());
    }

    let token = http::load_token().context(
        "set AGENT_SYNC_HARNESS_TOKEN or write a staff PAT to ~/.agent-sync/credentials",
    )?;
    let base = cfg.base_url.trim_end_matches('/');
    let url = format!("{base}/api/harness/publish");
    let response = http::post_json(&url, &token, &envelope)?;
    let status = response.status();
    let body = response.text().unwrap_or_default();
    if !status.is_success() {
        bail!("publish failed ({status}): {body}");
    }
    if !body.is_empty() {
        println!("{body}");
    }
    Ok(())
}

fn pilot_allowlist() -> std::collections::HashSet<&'static str> {
    PILOT_SKILLS
        .iter()
        .chain(PILOT_COMMANDS.iter())
        .copied()
        .collect()
}

fn scan_skill(skill_dir: &Path, name: &str) -> Result<Value> {
    let mut files_map: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    for entry in WalkDir::new(skill_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let rel = path
            .strip_prefix(skill_dir)
            .with_context(|| format!("strip prefix for {}", path.display()))?;
        let rel_str = rel.to_string_lossy().replace('\\', "/");
        let bytes = fs::read(path).with_context(|| format!("read {}", path.display()))?;
        files_map.insert(rel_str, bytes);
    }
    if files_map.is_empty() {
        bail!("skill '{name}' has no files at {}", skill_dir.display());
    }
    item_json("skill", name, files_map)
}

fn scan_command(command_path: &Path, name: &str) -> Result<Value> {
    let bytes =
        fs::read(command_path).with_context(|| format!("read {}", command_path.display()))?;
    let file_name = command_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("COMMAND.md")
        .to_owned();
    let mut files_map = BTreeMap::new();
    files_map.insert(file_name, bytes);
    item_json("command", name, files_map)
}

fn item_json(kind: &str, name: &str, files_map: BTreeMap<String, Vec<u8>>) -> Result<Value> {
    let content_hash = content_hash(&files_map);
    let mut files = Vec::new();
    for (path, bytes) in &files_map {
        let len = bytes.len() as u64;
        if len > MAX_INLINE_BYTES {
            bail!(
                "file '{path}' in {kind} '{name}' is {len} bytes; \
                 exceeds {MAX_INLINE_BYTES} inline limit (GCS path not implemented yet)"
            );
        }
        let sha = hex::encode(Sha256::digest(bytes));
        files.push(json!({
            "path": path,
            "sha256": sha,
            "inlineBase64": base64::engine::general_purpose::STANDARD.encode(bytes),
            "gcsPath": Value::Null,
        }));
    }
    Ok(json!({
        "kind": kind,
        "name": name,
        "contentHash": content_hash,
        "files": files,
    }))
}

/// Canonical content hash: sort paths, then sha256 of `path\0hex\n` lines.
fn content_hash(files: &BTreeMap<String, Vec<u8>>) -> String {
    let mut hasher = Sha256::new();
    for (path, bytes) in files {
        let file_hex = hex::encode(Sha256::digest(bytes));
        hasher.update(path.as_bytes());
        hasher.update([0]);
        hasher.update(file_hex.as_bytes());
        hasher.update(b"\n");
    }
    hex::encode(hasher.finalize())
}
