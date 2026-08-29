use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

#[test]
fn sync_installs_symlink_and_copy_with_cursor_overlay() {
    let workspace = tempdir().expect("temporary workspace");
    let dotfiles = workspace.path().join("dotfiles");
    let home = workspace.path().join("home");
    let target_root = workspace.path().join("targets");
    let skill = dotfiles.join("library/skills/demo");

    fs::create_dir_all(&skill).expect("skill directory");
    fs::create_dir_all(&home).expect("home directory");
    fs::write(
        skill.join("SKILL.md"),
        "---\nname: demo\ndescription: shared\nmetadata:\n  source: public\n---\n\n# Demo\n",
    )
    .expect("skill body");
    fs::write(
        skill.join("manifest.toml"),
        r#"
[overlays.cursor]
body_append = """

cursor only
"""

[overlays.cursor.frontmatter]
disable-model-invocation = true

[overlays.cursor.frontmatter.metadata]
source = "cursor"
"#,
    )
    .expect("manifest");

    let output = Command::new(env!("CARGO_BIN_EXE_agent-sync"))
        .args(["sync", "--root"])
        .arg(&target_root)
        .env("DOTFILES_DIR", &dotfiles)
        .env("HOME", &home)
        .output()
        .expect("run agent-sync");

    assert!(
        output.status.success(),
        "sync failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let claude = target_root.join(".claude/skills/andrew-demo");
    let cursor = target_root.join(".cursor/skills/andrew-demo");
    assert!(
        fs::symlink_metadata(&claude)
            .expect("claude install")
            .file_type()
            .is_symlink(),
        "Claude install should be a symlink"
    );
    assert!(
        fs::metadata(&cursor).expect("cursor install").is_dir(),
        "Cursor install should be a directory copy"
    );
    assert!(
        !fs::symlink_metadata(&cursor)
            .expect("cursor install")
            .file_type()
            .is_symlink(),
        "Cursor install must not be a symlink"
    );

    let cursor_body = fs::read_to_string(cursor.join("SKILL.md")).expect("Cursor wrapper contents");
    assert!(cursor_body.contains("disable-model-invocation: true"));
    assert!(cursor_body.contains("source: cursor"));
    assert!(cursor_body.contains("cursor only"));
}

#[test]
fn sync_merges_hook_packs_without_clobbering_handwritten_entries() {
    let workspace = tempdir().expect("temporary workspace");
    let dotfiles = workspace.path().join("dotfiles");
    let home = workspace.path().join("home");
    let target_root = workspace.path().join("targets");
    let pack = dotfiles.join("library/hooks/guards");
    fs::create_dir_all(&pack).expect("Hook pack directory");
    fs::create_dir_all(target_root.join(".cursor")).expect("Cursor target");
    fs::create_dir_all(target_root.join(".claude")).expect("Claude target");
    fs::create_dir_all(&home).expect("home directory");
    fs::write(pack.join("guard.sh"), "#!/bin/sh\nexit 0\n").expect("hook script");
    fs::write(
        pack.join("manifest.toml"),
        r#"
version = "2.1.0"
exclude = ["opencode", "pi"]

[hooks.cursor]
beforeShellExecution = [{ command = "./guard.sh", timeout = 5 }]

[hooks.claude]
PreToolUse = [{ matcher = "Bash", hooks = [{ type = "command", command = "./guard.sh" }] }]
"#,
    )
    .expect("Hook pack Manifest");
    fs::write(
        target_root.join(".cursor/hooks.json"),
        r#"{"version":1,"hooks":{"beforeShellExecution":[{"command":"manual.sh"}]}}"#,
    )
    .expect("Cursor config");
    fs::write(
        target_root.join(".claude/settings.json"),
        r#"{"model":"opus","hooks":{"PreToolUse":[{"matcher":"Read","hooks":[]}]}}"#,
    )
    .expect("Claude config");

    let output = Command::new(env!("CARGO_BIN_EXE_agent-sync"))
        .args(["sync", "--root"])
        .arg(&target_root)
        .env("DOTFILES_DIR", &dotfiles)
        .env("HOME", &home)
        .output()
        .expect("run agent-sync");
    assert!(
        output.status.success(),
        "sync failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let cursor_config: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(target_root.join(".cursor/hooks.json")).expect("Cursor hooks"),
    )
    .expect("valid Cursor hooks");
    let cursor_entries = cursor_config["hooks"]["beforeShellExecution"]
        .as_array()
        .expect("Cursor event array");
    assert_eq!(cursor_entries[0]["command"], "manual.sh");
    assert_eq!(
        cursor_entries[1]["_as"],
        "agent-sync:guards:2.1.0:beforeShellExecution:0"
    );
    assert!(cursor_entries[1]["command"]
        .as_str()
        .expect("rewritten command")
        .ends_with(".cursor/hooks/as-guards-guard.sh"));
    assert!(
        !fs::symlink_metadata(target_root.join(".cursor/hooks/as-guards-guard.sh"))
            .expect("Cursor hook script")
            .file_type()
            .is_symlink()
    );

    let claude_config: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(target_root.join(".claude/settings.json")).expect("Claude settings"),
    )
    .expect("valid Claude settings");
    assert_eq!(claude_config["model"], "opus");
    assert_eq!(
        claude_config["hooks"]["PreToolUse"][1]["_as"],
        "agent-sync:guards:2.1.0:PreToolUse:0"
    );
    assert!(
        fs::symlink_metadata(target_root.join(".claude/hooks/as-guards-guard.sh"))
            .expect("Claude hook script")
            .file_type()
            .is_symlink()
    );

    let verify = Command::new(env!("CARGO_BIN_EXE_agent-sync"))
        .args(["verify", "--root"])
        .arg(&target_root)
        .env("DOTFILES_DIR", &dotfiles)
        .env("HOME", &home)
        .output()
        .expect("run verify");
    assert!(
        verify.status.success(),
        "verify failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&verify.stdout),
        String::from_utf8_lossy(&verify.stderr)
    );
}

#[test]
fn local_tombstone_suppresses_public_item() {
    let workspace = tempdir().expect("temporary workspace");
    let dotfiles = workspace.path().join("dotfiles");
    let home = workspace.path().join("home");
    let target_root = workspace.path().join("targets");
    let public = dotfiles.join("library/skills/hidden");
    let tombstone = home.join("dotfiles-local/library/skills/hidden");
    fs::create_dir_all(&public).expect("public skill");
    fs::create_dir_all(&tombstone).expect("local tombstone");
    fs::write(public.join("SKILL.md"), "# Hidden\n").expect("public body");
    fs::write(tombstone.join(".agent-sync-tombstone"), "").expect("tombstone marker");

    let output = Command::new(env!("CARGO_BIN_EXE_agent-sync"))
        .args(["sync", "--root"])
        .arg(&target_root)
        .env("DOTFILES_DIR", &dotfiles)
        .env("HOME", &home)
        .output()
        .expect("run agent-sync");
    assert!(output.status.success());
    assert!(!target_root.join(".claude/skills/andrew-hidden").exists());
    assert!(!target_root.join(".cursor/skills/andrew-hidden").exists());
}

#[test]
fn sync_installs_pi_skills_commands_as_skills_agents_and_hooks() {
    let workspace = tempdir().expect("temporary workspace");
    let dotfiles = workspace.path().join("dotfiles");
    let home = workspace.path().join("home");
    let target_root = workspace.path().join("targets");

    let skill = dotfiles.join("library/skills/demo");
    fs::create_dir_all(&skill).expect("skill directory");
    fs::create_dir_all(&home).expect("home directory");
    fs::write(skill.join("SKILL.md"), "---\nname: demo\ndescription: skill\n---\n\n# Demo\n")
        .expect("skill body");

    let command = dotfiles.join("library/commands/democmd");
    fs::create_dir_all(&command).expect("command directory");
    fs::write(
        command.join("COMMAND.md"),
        "---\nname: democmd\ndescription: command as skill\n---\n\n# DemoCmd\n",
    )
    .expect("command body");

    let agent = dotfiles.join("library/agents/demo");
    fs::create_dir_all(&agent).expect("agent directory");
    fs::write(
        agent.join("AGENT.md"),
        "---\nname: demo\ndescription: agent\nmodel: opus\n---\n\n# Agent\n",
    )
    .expect("agent body");

    let pack = dotfiles.join("library/hooks/auto-sync");
    fs::create_dir_all(&pack).expect("hook pack");
    fs::write(
        pack.join("pull-if-stale.sh"),
        "#!/bin/sh\necho pull-if-stale\nexit 0\n",
    )
    .expect("hook script");
    fs::write(
        pack.join("manifest.toml"),
        r#"
version = "1.0.0"
exclude = ["opencode"]

[hooks.cursor]
sessionStart = [{ command = "./pull-if-stale.sh" }]

[hooks.claude]
SessionStart = [{ command = "./pull-if-stale.sh" }]

[hooks.pi]
session_start = [{ command = "./pull-if-stale.sh" }]
"#,
    )
    .expect("hook manifest");

    let output = Command::new(env!("CARGO_BIN_EXE_agent-sync"))
        .args(["sync", "--root"])
        .arg(&target_root)
        .env("DOTFILES_DIR", &dotfiles)
        .env("HOME", &home)
        .output()
        .expect("run agent-sync");
    assert!(
        output.status.success(),
        "sync failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        target_root.join(".pi/agent/skills/andrew-demo").exists(),
        "Pi skill missing"
    );
    assert!(
        target_root
            .join(".pi/agent/skills/andrew-democmd")
            .join("SKILL.md")
            .exists(),
        "Pi command-as-skill missing"
    );
    assert!(
        target_root.join(".pi/agent/agents/andrew-demo.md").exists(),
        "Pi agent missing"
    );
    let agent_body =
        fs::read_to_string(target_root.join(".pi/agent/agents/andrew-demo.md")).expect("agent");
    assert!(
        !agent_body.contains("model:"),
        "Pi agents should strip model alias, got: {agent_body}"
    );

    // Spike: Pi renamed hooks/ → extensions/; install under extensions/.
    let extensions = target_root.join(".pi/agent/extensions");
    assert!(extensions.is_dir(), "Pi extensions dir missing");
    let managed = extensions.join(".agent-sync-managed.json");
    assert!(managed.exists(), "Pi managed registry missing");
    let managed_text = fs::read_to_string(&managed).expect("managed registry");
    assert!(
        managed_text.contains("agent-sync:auto-sync"),
        "managed registry missing _as tag: {managed_text}"
    );

    let cursor_hooks = fs::read_to_string(target_root.join(".cursor/hooks.json")).expect("cursor hooks");
    assert!(
        cursor_hooks.contains("agent-sync:auto-sync"),
        "cursor hooks missing auto-sync _as: {cursor_hooks}"
    );
    let claude_settings =
        fs::read_to_string(target_root.join(".claude/settings.json")).expect("claude settings");
    assert!(
        claude_settings.contains("agent-sync:auto-sync"),
        "claude settings missing auto-sync _as: {claude_settings}"
    );

    let wrappers: Vec<_> = fs::read_dir(&extensions)
        .expect("read extensions")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".ts"))
        .collect();
    assert!(
        !wrappers.is_empty(),
        "expected at least one Pi .ts hook wrapper"
    );
    let wrapper_body = fs::read_to_string(extensions.join(&wrappers[0])).expect("wrapper");
    assert!(
        wrapper_body.contains("pull-if-stale"),
        "wrapper should exec pull-if-stale script, got: {wrapper_body}"
    );
    assert!(
        wrapper_body.contains("session_start"),
        "wrapper should register session_start, got: {wrapper_body}"
    );

    let verify = Command::new(env!("CARGO_BIN_EXE_agent-sync"))
        .args(["verify", "--root"])
        .arg(&target_root)
        .env("DOTFILES_DIR", &dotfiles)
        .env("HOME", &home)
        .output()
        .expect("run verify");
    assert!(
        verify.status.success(),
        "verify failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&verify.stdout),
        String::from_utf8_lossy(&verify.stderr)
    );
}

#[test]
fn hybrid_source_prefers_cache_over_public() {
    let workspace = tempdir().expect("temporary workspace");
    let dotfiles = workspace.path().join("dotfiles");
    let home = workspace.path().join("home");
    let target_root = workspace.path().join("targets");

    let public = dotfiles.join("library/skills/shared");
    fs::create_dir_all(&public).expect("public skill");
    fs::create_dir_all(&home).expect("home");
    fs::write(public.join("SKILL.md"), "# public\n").expect("public body");

    let cache = home.join(".agent-sync/cache/stable/library/skills/shared");
    fs::create_dir_all(&cache).expect("cache skill");
    fs::write(cache.join("SKILL.md"), "# cache\n").expect("cache body");

    let output = Command::new(env!("CARGO_BIN_EXE_agent-sync"))
        .args(["sync", "--source", "hybrid", "--root"])
        .arg(&target_root)
        .env("DOTFILES_DIR", &dotfiles)
        .env("HOME", &home)
        .output()
        .expect("run agent-sync");
    assert!(
        output.status.success(),
        "hybrid sync failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let installed = fs::read_to_string(target_root.join(".claude/skills/andrew-shared/SKILL.md"))
        .expect("installed skill");
    assert!(
        installed.contains("# cache"),
        "hybrid should install cache body, got: {installed}"
    );
}

#[test]
fn auto_sync_pack_from_library_merges_managed_prefix() {
    let workspace = tempdir().expect("temporary workspace");
    let dotfiles = workspace.path().join("dotfiles");
    let home = workspace.path().join("home");
    let target_root = workspace.path().join("targets");
    fs::create_dir_all(dotfiles.join("library")).expect("library");
    fs::create_dir_all(&home).expect("home");

    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../library/hooks/auto-sync");
    let dest = dotfiles.join("library/hooks/auto-sync");
    fs::create_dir_all(dest.parent().unwrap()).expect("hooks dir");
    copy_dir_recursive(&src, &dest).expect("copy auto-sync pack");

    let output = Command::new(env!("CARGO_BIN_EXE_agent-sync"))
        .args(["sync", "--root"])
        .arg(&target_root)
        .env("DOTFILES_DIR", &dotfiles)
        .env("HOME", &home)
        .output()
        .expect("run agent-sync");
    assert!(
        output.status.success(),
        "sync failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let cursor = fs::read_to_string(target_root.join(".cursor/hooks.json")).expect("cursor");
    assert!(
        cursor.contains("agent-sync:auto-sync"),
        "cursor missing agent-sync:auto-sync prefix: {cursor}"
    );
    let claude = fs::read_to_string(target_root.join(".claude/settings.json")).expect("claude");
    assert!(
        claude.contains("agent-sync:auto-sync"),
        "claude missing agent-sync:auto-sync prefix: {claude}"
    );
    let managed = fs::read_to_string(
        target_root.join(".pi/agent/extensions/.agent-sync-managed.json"),
    )
    .expect("pi managed");
    assert!(
        managed.contains("agent-sync:auto-sync"),
        "pi managed registry missing auto-sync: {managed}"
    );
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let to = dest.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_recursive(&entry.path(), &to)?;
        } else {
            fs::copy(entry.path(), to)?;
        }
    }
    Ok(())
}
