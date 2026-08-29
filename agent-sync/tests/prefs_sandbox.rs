use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::sync::Mutex;

use agent_sync::prefs::{self, LiveSync, Preferences};
use agent_sync::pull::{
    self, HarnessClient, ManifestBody, ManifestFetch, ManifestItem, PullConfig, PullStatus,
    RevisionFile, RevisionPayload,
};
use anyhow::Result;
use base64::Engine;
use sha2::{Digest, Sha256};
use tempfile::tempdir;

struct FakeClient {
    manifest: Mutex<Option<ManifestFetch>>,
    revisions: HashMap<(String, String), RevisionPayload>,
    prefs: Option<Preferences>,
}

impl FakeClient {
    fn with_manifest(
        fetch: ManifestFetch,
        revisions: HashMap<(String, String), RevisionPayload>,
    ) -> Self {
        Self {
            manifest: Mutex::new(Some(fetch)),
            revisions,
            prefs: None,
        }
    }
}

impl HarnessClient for FakeClient {
    fn get_manifest(&self, _channel: &str, if_none_match: Option<&str>) -> Result<ManifestFetch> {
        let fetch = self
            .manifest
            .lock()
            .unwrap()
            .take()
            .ok_or_else(|| anyhow::anyhow!("manifest already consumed"))?;
        match &fetch {
            ManifestFetch::NotModified => Ok(ManifestFetch::NotModified),
            ManifestFetch::Modified { etag, .. } => {
                if if_none_match == Some(etag.as_str()) {
                    Ok(ManifestFetch::NotModified)
                } else {
                    Ok(fetch)
                }
            }
        }
    }

    fn get_revision(&self, item_id: &str, content_hash: &str) -> Result<RevisionPayload> {
        self.revisions
            .get(&(item_id.to_owned(), content_hash.to_owned()))
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("missing revision {item_id}/{content_hash}"))
    }

    fn get_preferences(&self) -> Result<Option<Preferences>> {
        Ok(self.prefs.clone())
    }
}

fn skill_revision(name: &str, body: &str) -> (String, RevisionPayload) {
    let bytes = body.as_bytes();
    let sha = hex::encode(Sha256::digest(bytes));
    let content_hash = hex::encode(Sha256::digest(format!("SKILL.md\0{sha}\n").as_bytes()));
    let payload = RevisionPayload {
        kind: "skill".to_owned(),
        name: name.to_owned(),
        content_hash: content_hash.clone(),
        files: vec![RevisionFile {
            path: "SKILL.md".to_owned(),
            sha256: sha,
            inline_base64: Some(base64::engine::general_purpose::STANDARD.encode(bytes)),
            gcs_path: None,
        }],
    };
    (content_hash, payload)
}

#[test]
fn pull_skips_when_live_sync_off() {
    let workspace = tempdir().expect("workspace");
    let home = workspace.path().join("home");
    fs::create_dir_all(home.join(".agent-sync")).expect("agent-sync dir");

    let mut prefs = Preferences::default();
    prefs.live_sync = LiveSync {
        mode: "off".to_owned(),
        channel: "stable".to_owned(),
    };
    prefs::save(&home, &prefs).expect("save prefs");

    let client = FakeClient::with_manifest(
        ManifestFetch::Modified {
            etag: "\"should-not-fetch\"".to_owned(),
            body: ManifestBody {
                revision_number: 1,
                items: vec![],
            },
        },
        HashMap::new(),
    );

    let status = pull::pull_with_client(
        &PullConfig {
            home: home.clone(),
            base_url: "http://example.test".to_owned(),
            channel: "stable".to_owned(),
            if_stale: true,
        },
        &client,
    )
    .expect("pull");
    assert_eq!(status, PullStatus::Skipped);
    assert!(
        !home.join(".agent-sync/cache/stable").exists(),
        "cache must not be created when live_sync is off"
    );
}

#[test]
fn pull_applies_tombstones_and_pins() {
    let workspace = tempdir().expect("workspace");
    let home = workspace.path().join("home");
    fs::create_dir_all(&home).expect("home");

    let (tip_hash, tip_rev) = skill_revision("pinned", "# tip\n");
    let (pin_hash, pin_rev) = skill_revision("pinned", "# pinned\n");
    let (gone_hash, gone_rev) = skill_revision("gone", "# gone\n");

    let keep_id = "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa".to_owned();
    let gone_id = "bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb".to_owned();

    let mut prefs = Preferences::default();
    prefs.tombstones.push(gone_id.clone());
    prefs.pins.insert(keep_id.clone(), pin_hash.clone());
    prefs::save(&home, &prefs).expect("save prefs");

    let mut revisions = HashMap::new();
    revisions.insert((keep_id.clone(), tip_hash.clone()), tip_rev);
    revisions.insert((keep_id.clone(), pin_hash.clone()), pin_rev);
    revisions.insert((gone_id.clone(), gone_hash.clone()), gone_rev);

    let client = FakeClient::with_manifest(
        ManifestFetch::Modified {
            etag: "\"etag-1\"".to_owned(),
            body: ManifestBody {
                revision_number: 2,
                items: vec![
                    ManifestItem {
                        item_id: keep_id,
                        kind: "skill".to_owned(),
                        name: "pinned".to_owned(),
                        content_hash: tip_hash,
                    },
                    ManifestItem {
                        item_id: gone_id,
                        kind: "skill".to_owned(),
                        name: "gone".to_owned(),
                        content_hash: gone_hash,
                    },
                ],
            },
        },
        revisions,
    );

    let status = pull::pull_with_client(
        &PullConfig {
            home: home.clone(),
            base_url: "http://example.test".to_owned(),
            channel: "stable".to_owned(),
            if_stale: false,
        },
        &client,
    )
    .expect("pull");
    assert_eq!(status, PullStatus::Updated);

    let skill = home.join(".agent-sync/cache/stable/library/skills/pinned/SKILL.md");
    assert_eq!(fs::read_to_string(skill).expect("pinned body"), "# pinned\n");
    assert!(
        !home
            .join(".agent-sync/cache/stable/library/skills/gone")
            .exists(),
        "tombstoned item must not land in cache"
    );
}

#[test]
fn pull_writes_default_preferences_when_missing() {
    let workspace = tempdir().expect("workspace");
    let home = workspace.path().join("home");
    fs::create_dir_all(&home).expect("home");

    let client = FakeClient::with_manifest(
        ManifestFetch::Modified {
            etag: "\"e\"".to_owned(),
            body: ManifestBody {
                revision_number: 1,
                items: vec![],
            },
        },
        HashMap::new(),
    );

    pull::pull_with_client(
        &PullConfig {
            home: home.clone(),
            base_url: "http://example.test".to_owned(),
            channel: "stable".to_owned(),
            if_stale: false,
        },
        &client,
    )
    .expect("pull");

    let prefs = prefs::load(&home).expect("load");
    assert_eq!(prefs.live_sync.mode, "stale_check");
    assert_eq!(prefs.live_sync.channel, "stable");
    assert!(prefs::preferences_path(&home).is_file());
}

#[test]
fn doctor_warns_missing_token_when_url_set() {
    let workspace = tempdir().expect("workspace");
    let home = workspace.path().join("home");
    let dotfiles = workspace.path().join("dotfiles");
    fs::create_dir_all(dotfiles.join("library")).expect("library");
    fs::create_dir_all(&home).expect("home");

    let output = Command::new(env!("CARGO_BIN_EXE_agent-sync"))
        .args(["doctor", "--root"])
        .arg(&home)
        .env("HOME", &home)
        .env("DOTFILES_DIR", &dotfiles)
        .env("AGENT_SYNC_HARNESS_URL", "https://admin.example.test")
        .env_remove("AGENT_SYNC_HARNESS_TOKEN")
        .output()
        .expect("run doctor");

    assert!(
        output.status.success(),
        "doctor should succeed with only a warning:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("AGENT_SYNC_HARNESS_URL") && stdout.contains("no token"),
        "expected harness URL/token warning, got: {stdout}"
    );
}

#[test]
fn verify_compares_installed_to_cache_when_present() {
    let workspace = tempdir().expect("workspace");
    let home = workspace.path().join("home");
    let dotfiles = workspace.path().join("dotfiles");
    let targets = workspace.path().join("targets");
    fs::create_dir_all(dotfiles.join("library/skills")).expect("library");
    fs::create_dir_all(&home).expect("home");

    let cache_skill = home.join(".agent-sync/cache/stable/library/skills/demo");
    fs::create_dir_all(&cache_skill).expect("cache skill");
    fs::write(cache_skill.join("SKILL.md"), "# cache-demo\n").expect("cache body");

    let installed = targets.join(".claude/skills/andrew-demo");
    fs::create_dir_all(&installed).expect("installed");
    fs::write(installed.join("SKILL.md"), "# wrong\n").expect("installed body");

    let output = Command::new(env!("CARGO_BIN_EXE_agent-sync"))
        .args(["verify", "--root"])
        .arg(&targets)
        .env("HOME", &home)
        .env("DOTFILES_DIR", &dotfiles)
        .output()
        .expect("run verify");

    assert!(
        !output.status.success(),
        "verify must fail on cache mismatch:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let err = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        err.contains("harness cache mismatch"),
        "expected cache mismatch error, got: {err}"
    );
}
