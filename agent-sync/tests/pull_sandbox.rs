use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

use agent_sync::config::Config;
use agent_sync::library::{ItemSource, Library, SourceMode};
use agent_sync::pull::{
    self, channel_cache_dir, HarnessClient, ManifestBody, ManifestFetch, ManifestItem, PullConfig,
    PullStatus, RevisionFile, RevisionPayload,
};
use anyhow::Result;
use base64::Engine;
use sha2::{Digest, Sha256};
use tempfile::tempdir;

struct FakeClient {
    manifest: Mutex<Option<ManifestFetch>>,
    revisions: HashMap<(String, String), RevisionPayload>,
}

impl FakeClient {
    fn with_manifest(
        fetch: ManifestFetch,
        revisions: HashMap<(String, String), RevisionPayload>,
    ) -> Self {
        Self {
            manifest: Mutex::new(Some(fetch)),
            revisions,
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

fn write_skill(root: &Path, name: &str, body: &str) {
    let dir = root.join("skills").join(name);
    fs::create_dir_all(&dir).expect("skill dir");
    fs::write(dir.join("SKILL.md"), body).expect("skill body");
}

fn test_config(dotfiles: &Path, home: &Path) -> Config {
    Config {
        public_library: dotfiles.join("library"),
        local_library: home.join("dotfiles-local/library"),
        target_home: home.to_path_buf(),
        state_file: home.join(".agent-sync-state.json"),
        wrapper_root: home.join(".agent-sync/wrappers"),
        backup_root: dotfiles.join(".agent-sync-backups"),
        dotfiles_dir: dotfiles.to_path_buf(),
        home: home.to_path_buf(),
        owner_prefix: "andrew".to_owned(),
        harness_channel: "stable".to_owned(),
    }
}

#[test]
fn pull_writes_cache_layout_and_etag() {
    let workspace = tempdir().expect("workspace");
    let home = workspace.path().join("home");
    fs::create_dir_all(&home).expect("home");

    let (hash, revision) = skill_revision("ship-feature", "# ship\n");
    let item_id = "11111111-1111-1111-1111-111111111111".to_owned();
    let etag = "\"etag-v1\"".to_owned();
    let mut revisions = HashMap::new();
    revisions.insert((item_id.clone(), hash.clone()), revision);

    let client = FakeClient::with_manifest(
        ManifestFetch::Modified {
            etag: etag.clone(),
            body: ManifestBody {
                revision_number: 3,
                items: vec![ManifestItem {
                    item_id,
                    kind: "skill".to_owned(),
                    name: "ship-feature".to_owned(),
                    content_hash: hash,
                }],
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

    let cache = channel_cache_dir(&home, "stable");
    assert_eq!(
        fs::read_to_string(cache.join("ETAG")).expect("ETAG").trim(),
        etag.as_str()
    );
    let skill = cache.join("library/skills/ship-feature/SKILL.md");
    assert_eq!(fs::read_to_string(skill).expect("skill"), "# ship\n");
}

#[test]
fn pull_if_stale_keeps_cache_on_304() {
    let workspace = tempdir().expect("workspace");
    let home = workspace.path().join("home");
    let cache = channel_cache_dir(&home, "stable");
    let library = cache.join("library/skills/kept");
    fs::create_dir_all(&library).expect("prior cache");
    fs::write(library.join("SKILL.md"), "# prior\n").expect("prior skill");
    fs::write(cache.join("ETAG"), "\"etag-prior\"\n").expect("prior etag");

    let client = FakeClient::with_manifest(
        ManifestFetch::Modified {
            etag: "\"etag-prior\"".to_owned(),
            body: ManifestBody {
                revision_number: 99,
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
    assert_eq!(status, PullStatus::NotModified);
    assert_eq!(
        fs::read_to_string(library.join("SKILL.md")).expect("kept"),
        "# prior\n"
    );
    assert_eq!(
        fs::read_to_string(cache.join("ETAG")).expect("etag").trim(),
        "\"etag-prior\""
    );
}

#[test]
fn pull_hash_mismatch_keeps_prior_cache() {
    let workspace = tempdir().expect("workspace");
    let home = workspace.path().join("home");
    let cache = channel_cache_dir(&home, "stable");
    let library = cache.join("library/skills/kept");
    fs::create_dir_all(&library).expect("prior cache");
    fs::write(library.join("SKILL.md"), "# prior\n").expect("prior skill");
    fs::write(cache.join("ETAG"), "\"etag-prior\"\n").expect("prior etag");

    let item_id = "22222222-2222-2222-2222-222222222222".to_owned();
    let bad_hash = "deadbeef".to_owned();
    let bytes = b"# corrupt\n";
    let mut revisions = HashMap::new();
    revisions.insert(
        (item_id.clone(), bad_hash.clone()),
        RevisionPayload {
            kind: "skill".to_owned(),
            name: "broken".to_owned(),
            content_hash: bad_hash.clone(),
            files: vec![RevisionFile {
                path: "SKILL.md".to_owned(),
                sha256: "0000000000000000000000000000000000000000000000000000000000000000"
                    .to_owned(),
                inline_base64: Some(base64::engine::general_purpose::STANDARD.encode(bytes)),
                gcs_path: None,
            }],
        },
    );

    let client = FakeClient::with_manifest(
        ManifestFetch::Modified {
            etag: "\"etag-new\"".to_owned(),
            body: ManifestBody {
                revision_number: 4,
                items: vec![ManifestItem {
                    item_id,
                    kind: "skill".to_owned(),
                    name: "broken".to_owned(),
                    content_hash: bad_hash,
                }],
            },
        },
        revisions,
    );

    let err = pull::pull_with_client(
        &PullConfig {
            home: home.clone(),
            base_url: "http://example.test".to_owned(),
            channel: "stable".to_owned(),
            if_stale: false,
        },
        &client,
    )
    .expect_err("hash mismatch must fail");
    assert!(
        err.to_string().contains("sha256 mismatch"),
        "unexpected error: {err}"
    );
    assert_eq!(
        fs::read_to_string(library.join("SKILL.md")).expect("prior kept"),
        "# prior\n"
    );
    assert_eq!(
        fs::read_to_string(cache.join("ETAG")).expect("etag").trim(),
        "\"etag-prior\""
    );
    assert!(!cache.join("library/skills/broken").exists());
}

#[test]
fn hybrid_prefers_local_then_cache_then_public() {
    let workspace = tempdir().expect("workspace");
    let dotfiles = workspace.path().join("dotfiles");
    let home = workspace.path().join("home");
    fs::create_dir_all(dotfiles.join("library")).expect("public lib");
    fs::create_dir_all(home.join("dotfiles-local/library")).expect("local lib");

    write_skill(&dotfiles.join("library"), "alpha", "# public-alpha\n");
    write_skill(&dotfiles.join("library"), "beta", "# public-beta\n");
    write_skill(&dotfiles.join("library"), "gamma", "# public-gamma\n");

    let cache_lib = home.join(".agent-sync/cache/stable/library");
    write_skill(&cache_lib, "beta", "# cache-beta\n");
    write_skill(&cache_lib, "delta", "# cache-delta\n");

    write_skill(
        &home.join("dotfiles-local/library"),
        "alpha",
        "# local-alpha\n",
    );
    let tomb = home.join("dotfiles-local/library/skills/gamma");
    fs::create_dir_all(&tomb).expect("tomb dir");
    fs::write(tomb.join(".agent-sync-tombstone"), "").expect("tombstone");

    let config = test_config(&dotfiles, &home);
    let library = Library::scan_with_source(&config, SourceMode::Hybrid).expect("scan");

    let by_name: HashMap<_, _> = library
        .items
        .iter()
        .map(|item| (item.name.as_str(), item))
        .collect();

    assert_eq!(by_name["alpha"].source, ItemSource::Local);
    assert!(fs::read_to_string(by_name["alpha"].path.join("SKILL.md"))
        .unwrap()
        .contains("local-alpha"));

    assert_eq!(by_name["beta"].source, ItemSource::Cache);
    assert!(fs::read_to_string(by_name["beta"].path.join("SKILL.md"))
        .unwrap()
        .contains("cache-beta"));

    assert_eq!(by_name["delta"].source, ItemSource::Cache);
    assert!(!by_name.contains_key("gamma"));
    assert!(library.tombstones.iter().any(|t| t == "skills/gamma"));
}
