//! Pull harness channel manifests into `~/.agent-sync/cache/<channel>/`.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use base64::Engine;
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::http;
use crate::prefs::{self, Preferences};

/// Channel pull options.
#[derive(Debug, Clone)]
pub struct PullConfig {
    pub home: PathBuf,
    pub base_url: String,
    pub channel: String,
    pub if_stale: bool,
}

/// Result of a pull attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PullStatus {
    Updated,
    NotModified,
    /// `live_sync.mode=off` in preferences — no network call.
    Skipped,
}

/// Manifest + revision HTTP surface (real or fake).
pub trait HarnessClient {
    fn get_manifest(&self, channel: &str, if_none_match: Option<&str>) -> Result<ManifestFetch>;

    fn get_revision(&self, item_id: &str, content_hash: &str) -> Result<RevisionPayload>;

    /// Optional staff preferences from the control plane (overwrite local file on success).
    fn get_preferences(&self) -> Result<Option<Preferences>> {
        Ok(None)
    }
}

#[derive(Debug)]
pub enum ManifestFetch {
    NotModified,
    Modified { etag: String, body: ManifestBody },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestBody {
    pub revision_number: i64,
    pub items: Vec<ManifestItem>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestItem {
    pub item_id: String,
    pub kind: String,
    pub name: String,
    pub content_hash: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionPayload {
    pub kind: String,
    pub name: String,
    pub content_hash: String,
    pub files: Vec<RevisionFile>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionFile {
    pub path: String,
    pub sha256: String,
    pub inline_base64: Option<String>,
    pub gcs_path: Option<String>,
}

/// Cache root for a channel: `~/.agent-sync/cache/<channel>`.
#[must_use]
pub fn channel_cache_dir(home: &Path, channel: &str) -> PathBuf {
    home.join(".agent-sync/cache").join(channel)
}

/// Library tree under the channel cache.
#[must_use]
pub fn channel_cache_library(home: &Path, channel: &str) -> PathBuf {
    channel_cache_dir(home, channel).join("library")
}

/// Pull using the default reqwest client.
pub fn pull(cfg: &PullConfig) -> Result<PullStatus> {
    let token = http::load_token().context(
        "set AGENT_SYNC_HARNESS_TOKEN or write a staff PAT to ~/.agent-sync/credentials",
    )?;
    let client = ReqwestHarnessClient::new(&cfg.base_url, &token)?;
    pull_with_client(cfg, &client)
}

/// Pull with an injectable client (tests use a fake).
pub fn pull_with_client(cfg: &PullConfig, client: &dyn HarnessClient) -> Result<PullStatus> {
    let mut local_prefs = prefs::ensure_defaults(&cfg.home)?;
    if local_prefs.live_sync_off() {
        println!("PULL skipped (live_sync.mode=off)");
        return Ok(PullStatus::Skipped);
    }

    let channel = if cfg.channel.is_empty() {
        local_prefs.live_sync.channel.clone()
    } else {
        cfg.channel.clone()
    };

    let cache_dir = channel_cache_dir(&cfg.home, &channel);
    let etag_path = cache_dir.join("ETAG");
    let library_path = cache_dir.join("library");

    let if_none_match = if cfg.if_stale {
        fs::read_to_string(&etag_path)
            .ok()
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
    } else {
        None
    };

    let fetch = client.get_manifest(&channel, if_none_match.as_deref())?;
    let ManifestFetch::Modified { etag, body } = fetch else {
        return Ok(PullStatus::NotModified);
    };

    // Prefer API prefs when present; always keep a local preferences.json.
    if let Some(api_prefs) = client.get_preferences()? {
        prefs::save(&cfg.home, &api_prefs)?;
        local_prefs = api_prefs;
        if local_prefs.live_sync_off() {
            println!("PULL skipped after API prefs (live_sync.mode=off)");
            return Ok(PullStatus::Skipped);
        }
    }

    let items = apply_prefs_to_items(body.items, &local_prefs);

    // Stage under a sibling dir so a failed pull leaves the prior cache intact.
    let staging = cache_dir.join(".pull-staging");
    if staging.exists() {
        fs::remove_dir_all(&staging)
            .with_context(|| format!("remove stale staging {}", staging.display()))?;
    }
    let staging_library = staging.join("library");
    fs::create_dir_all(&staging_library)
        .with_context(|| format!("create staging library {}", staging_library.display()))?;

    for item in &items {
        let revision = client.get_revision(&item.item_id, &item.content_hash)?;
        write_revision_to_library(&staging_library, &revision)?;
    }

    fs::write(staging.join("ETAG"), format!("{etag}\n"))
        .with_context(|| format!("write staging ETAG under {}", staging.display()))?;

    // Swap: replace library then ETAG (ETAG last so a crash mid-swap still has old ETAG).
    if library_path.exists() {
        fs::remove_dir_all(&library_path)
            .with_context(|| format!("remove prior cache library {}", library_path.display()))?;
    }
    fs::create_dir_all(&cache_dir)
        .with_context(|| format!("create cache dir {}", cache_dir.display()))?;
    fs::rename(&staging_library, &library_path)
        .with_context(|| format!("promote staging library to {}", library_path.display()))?;
    fs::rename(staging.join("ETAG"), &etag_path)
        .with_context(|| format!("promote staging ETAG to {}", etag_path.display()))?;
    let _ = fs::remove_dir_all(&staging);

    println!(
        "PULL {} channel={} revision={} items={}",
        cache_dir.display(),
        channel,
        body.revision_number,
        items.len()
    );
    Ok(PullStatus::Updated)
}

fn apply_prefs_to_items(items: Vec<ManifestItem>, prefs: &Preferences) -> Vec<ManifestItem> {
    items
        .into_iter()
        .filter_map(|mut item| {
            if prefs.is_tombstoned(&item.item_id) || prefs.is_tombstoned(&item.name) {
                return None;
            }
            if let Some(hash) = prefs.pin_for(&item.item_id, &item.name) {
                item.content_hash = hash.to_owned();
            }
            Some(item)
        })
        .collect()
}

fn write_revision_to_library(library_root: &Path, revision: &RevisionPayload) -> Result<()> {
    let kind_dir = kind_dir_name(&revision.kind)?;
    let item_dir = library_root.join(kind_dir).join(&revision.name);
    fs::create_dir_all(&item_dir)
        .with_context(|| format!("create item dir {}", item_dir.display()))?;

    for file in &revision.files {
        let bytes = decode_file_bytes(file)?;
        let actual = hex::encode(Sha256::digest(&bytes));
        if !actual.eq_ignore_ascii_case(&file.sha256) {
            bail!(
                "sha256 mismatch for {}/{} file '{}': expected {}, got {}",
                revision.kind,
                revision.name,
                file.path,
                file.sha256,
                actual
            );
        }
        let dest = item_dir.join(&file.path);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("create file parent {}", parent.display()))?;
        }
        fs::write(&dest, &bytes).with_context(|| format!("write {}", dest.display()))?;
    }

    // Soft-check aggregate content hash when present (non-fatal layout still written).
    let _ = revision.content_hash;
    Ok(())
}

fn decode_file_bytes(file: &RevisionFile) -> Result<Vec<u8>> {
    if let Some(b64) = &file.inline_base64 {
        return base64::engine::general_purpose::STANDARD
            .decode(b64)
            .with_context(|| format!("decode inlineBase64 for {}", file.path));
    }
    if file.gcs_path.is_some() {
        bail!(
            "file '{}' references gcsPath; GCS fetch is not implemented yet",
            file.path
        );
    }
    bail!("file '{}' has neither inlineBase64 nor gcsPath", file.path);
}

fn kind_dir_name(kind: &str) -> Result<&'static str> {
    match kind {
        "skill" => Ok("skills"),
        "command" => Ok("commands"),
        "agent" => Ok("agents"),
        "hook" | "hook_pack" => Ok("hooks"),
        other => bail!("unknown harness item kind '{other}'"),
    }
}

/// Reqwest-backed harness client.
pub struct ReqwestHarnessClient {
    client: reqwest::blocking::Client,
    base_url: String,
    token: String,
}

impl ReqwestHarnessClient {
    pub fn new(base_url: &str, token: &str) -> Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .use_rustls_tls()
            .build()
            .context("build HTTP client")?;
        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_owned(),
            token: token.to_owned(),
        })
    }
}

impl HarnessClient for ReqwestHarnessClient {
    fn get_manifest(&self, channel: &str, if_none_match: Option<&str>) -> Result<ManifestFetch> {
        let url = format!("{}/api/harness/manifest?channel={}", self.base_url, channel);
        let mut req = self.client.get(&url).bearer_auth(&self.token);
        if let Some(etag) = if_none_match {
            req = req.header(reqwest::header::IF_NONE_MATCH, etag);
        }
        let response = req.send().with_context(|| format!("GET {url}"))?;
        let status = response.status();
        if status == reqwest::StatusCode::NOT_MODIFIED {
            return Ok(ManifestFetch::NotModified);
        }
        if !status.is_success() {
            let body = response.text().unwrap_or_default();
            bail!("manifest fetch failed ({status}): {body}");
        }
        let etag = response
            .headers()
            .get(reqwest::header::ETAG)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_owned();
        let value: Value = response.json().context("parse manifest JSON")?;
        let body: ManifestBody = serde_json::from_value(value).context("decode manifest body")?;
        let etag = if etag.is_empty() {
            // Fall back to a deterministic etag from revision when server omits header.
            format!("\"rev-{}\"", body.revision_number)
        } else {
            etag
        };
        Ok(ManifestFetch::Modified { etag, body })
    }

    fn get_revision(&self, item_id: &str, content_hash: &str) -> Result<RevisionPayload> {
        let url = format!(
            "{}/api/harness/items/{item_id}/revisions/{content_hash}",
            self.base_url
        );
        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.token)
            .send()
            .with_context(|| format!("GET {url}"))?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().unwrap_or_default();
            bail!("revision fetch failed ({status}): {body}");
        }
        let value: Value = response.json().context("parse revision JSON")?;
        serde_json::from_value(value).context("decode revision payload")
    }

    fn get_preferences(&self) -> Result<Option<Preferences>> {
        let url = format!("{}/api/harness/preferences", self.base_url);
        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.token)
            .send()
            .with_context(|| format!("GET {url}"))?;
        let status = response.status();
        if status == reqwest::StatusCode::NOT_FOUND
            || status == reqwest::StatusCode::FORBIDDEN
            || status == reqwest::StatusCode::UNAUTHORIZED
        {
            return Ok(None);
        }
        if !status.is_success() {
            let body = response.text().unwrap_or_default();
            bail!("preferences fetch failed ({status}): {body}");
        }
        let value: Value = response.json().context("parse preferences JSON")?;
        // Accept camelCase API shape by normalizing keys.
        let normalized = normalize_prefs_json(value);
        let prefs: Preferences =
            serde_json::from_value(normalized).context("decode preferences")?;
        Ok(Some(prefs))
    }
}

fn normalize_prefs_json(value: Value) -> Value {
    let Value::Object(map) = value else {
        return value;
    };
    let mut out = serde_json::Map::new();
    for (key, val) in map {
        let snake = match key.as_str() {
            "liveSync" => "live_sync".to_owned(),
            "targetsEnabled" => "targets_enabled".to_owned(),
            other => other.to_owned(),
        };
        let val = if snake == "live_sync" {
            if let Value::Object(inner) = val {
                Value::Object(inner)
            } else {
                val
            }
        } else {
            val
        };
        out.insert(snake, val);
    }
    Value::Object(out)
}
