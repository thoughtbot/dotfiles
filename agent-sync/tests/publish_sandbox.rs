use std::fs;
use std::path::Path;

use agent_sync::publish;
use tempfile::tempdir;

#[test]
fn publish_rejects_item_outside_allowlist() {
    let workspace = tempdir().expect("temporary workspace");
    let platform_root = workspace.path();
    fs::create_dir_all(platform_root.join(".claude/skills")).expect("skills dir");

    let err = publish::build_envelope(platform_root, &["not-in-pilot".to_owned()]).unwrap_err();
    assert!(
        err.to_string().contains("allowlist"),
        "error should mention allowlist, got: {err}"
    );
}

#[test]
fn build_envelope_includes_only_requested_pilot_items() {
    let workspace = tempdir().expect("temporary workspace");
    let platform_root = workspace.path();
    write_skill(platform_root, "ship-feature", "# ship\n");
    write_skill(platform_root, "other-skill", "# other\n");
    write_command(platform_root, "plan-create", "# plan create\n");
    write_command(platform_root, "create-issues", "# not pilot\n");

    let envelope = publish::build_envelope(
        platform_root,
        &["ship-feature".to_owned(), "plan-create".to_owned()],
    )
    .expect("envelope");

    let items = envelope["items"].as_array().expect("items array");
    let names: Vec<(String, String)> = items
        .iter()
        .map(|item| {
            (
                item["kind"].as_str().unwrap().to_owned(),
                item["name"].as_str().unwrap().to_owned(),
            )
        })
        .collect();

    assert!(names.contains(&("skill".to_owned(), "ship-feature".to_owned())));
    assert!(names.contains(&("command".to_owned(), "plan-create".to_owned())));
    assert!(!names.iter().any(|(_, name)| name == "other-skill"));
    assert!(!names.iter().any(|(_, name)| name == "create-issues"));
    assert_eq!(names.len(), 2);

    assert!(envelope["idempotencyKey"].as_str().is_some());
    let ship = items
        .iter()
        .find(|item| item["name"] == "ship-feature")
        .expect("ship-feature item");
    assert!(ship["contentHash"].as_str().is_some());
    let files = ship["files"].as_array().expect("files");
    assert!(!files.is_empty());
    assert!(files[0]["inlineBase64"].as_str().is_some());
    assert!(files[0]["sha256"].as_str().is_some());
}

fn write_skill(platform_root: &Path, name: &str, body: &str) {
    let dir = platform_root.join(".claude/skills").join(name);
    fs::create_dir_all(&dir).expect("skill dir");
    fs::write(dir.join("SKILL.md"), body).expect("skill body");
}

fn write_command(platform_root: &Path, name: &str, body: &str) {
    let dir = platform_root.join(".claude/commands");
    fs::create_dir_all(&dir).expect("commands dir");
    fs::write(dir.join(format!("{name}.md")), body).expect("command body");
}
