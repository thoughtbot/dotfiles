//! Shared HTTP helpers for harness control-plane calls.
//! Token: `AGENT_SYNC_HARNESS_TOKEN` or `~/.agent-sync/credentials`.

use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use serde_json::Value;

/// Resolve staff PAT from env or credentials file.
pub fn load_token() -> Result<String> {
    if let Ok(token) = env::var("AGENT_SYNC_HARNESS_TOKEN") {
        let trimmed = token.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_owned());
        }
    }
    read_credentials_token()
}

/// True when env or `~/.agent-sync/credentials` yields a non-empty token.
#[must_use]
pub fn token_present() -> bool {
    load_token().is_ok()
}

/// True when `AGENT_SYNC_HARNESS_URL` is set to a non-empty value.
#[must_use]
pub fn harness_url_set() -> bool {
    env::var("AGENT_SYNC_HARNESS_URL")
        .ok()
        .is_some_and(|v| !v.trim().is_empty())
}

/// Read token from `~/.agent-sync/credentials` (trimmed plaintext).
pub fn read_credentials_token() -> Result<String> {
    let path = credentials_path()?;
    let raw = fs::read_to_string(&path)
        .with_context(|| format!("read credentials at {}", path.display()))?;
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        bail!("credentials file is empty: {}", path.display());
    }
    Ok(trimmed.to_owned())
}

fn credentials_path() -> Result<PathBuf> {
    let home = env::var_os("HOME").context("HOME is not set")?;
    Ok(PathBuf::from(home).join(".agent-sync/credentials"))
}

/// POST JSON with Bearer auth.
pub fn post_json(url: &str, token: &str, body: &Value) -> Result<reqwest::blocking::Response> {
    let client = reqwest::blocking::Client::builder()
        .use_rustls_tls()
        .build()
        .context("build HTTP client")?;
    client
        .post(url)
        .bearer_auth(token)
        .json(body)
        .send()
        .with_context(|| format!("POST {url}"))
}
