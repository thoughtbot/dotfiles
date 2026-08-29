# Agent skill library sync

Personal dotfiles context for an agent-neutral skill/command/agent library and the tool that fans it out to coding agents.

## Language

**Library**:
The single agent-neutral tree in the repo that owns skills, commands, agents, and hooks before any Target sees them (`library/{skills,commands,agents,hooks}/`).
_Avoid_: Claude tree, canonical Claude, source of truth (ambiguous), package

**Target**:
A coding agent the Sync tool can install into. v1: Claude Code, Cursor, OpenCode, Pi.
_Avoid_: Agent (overloaded with subagent files), platform, harness, `.agents` as a Fan-out Target (npx territory)

**Wrapper**:
The thin, Target-specific layer generated around a shared body (frontmatter overlay, optional body_append) — not a full rewrite of the skill.
_Avoid_: Template expansion, fork, variant skill

**Overlay**:
The Target-keyed patch in a Manifest applied onto the shared body to produce a Wrapper (deep-merge frontmatter; optional body_append).
_Avoid_: template, variant, fork, patch file (ambiguous with git)

**Fan-out**:
Installing one Library item into one or more Targets via generate-then-link-or-copy, with install basename `andrew-<name>` (first-party) or `vendor-<origin>-<name>`.
_Avoid_: Sync (too broad alone), mirror, rsync

**Sync tool**:
The `agent-sync` CLI that generates Wrappers and fans the Library out to Targets; coexists with `npx skills` for third-party installs. Commands: `sync`, `verify`, `list`, `migrate`, `doctor`.
_Avoid_: sync-ai-assistants (deleted), skills CLI (ambiguous with npx)

**Manifest**:
Per-item opt-in `manifest.toml` in the Library that names exclusions, overlays, and (for hooks) pack entrypoints. Missing Manifest means fan out to all valid Targets with no overlay.
_Avoid_: package.json, skills-lock, skill.yaml, manifest.json

**Hook pack**:
Versioned hook scripts plus Target entrypoint templates in the Library that Fan-out installs via tagged `_as` merge into Cursor/Claude configs.
_Avoid_: hooks.json alone (that's machine-local merge state)

**Vendor skill**:
A third-party or product-managed skill kept under `library/skills/vendor/<origin>/` and Fan-out as `vendor-<origin>-<name>`.
_Avoid_: skills-cursor (do not manage), builtin, forked skill

**Tombstone**:
A local-only marker under `~/dotfiles-local/library/` that skips Fan-out for a public `(kind, name)` on this machine without deleting the public item.
_Avoid_: delete, gitignore (different mechanism)

**Harness tombstone**:
Staff preference (prefs `tombstones`) or `~/dotfiles-local/library/<kind>/<name>/.agent-sync-tombstone` that suppresses an org catalog item from pull/Fan-out.
_Avoid_: deleting the public Library item, gitignore

**Harness pin**:
Staff preference that freezes an item to a content hash even when the `stable` channel tip moves.
_Avoid_: editing cache files by hand, pinning via git checkout

**Harness live_sync**:
Client auto-pull policy; default `mode=stale_check` on channel `stable`. Set `mode=off` in `~/.agent-sync/preferences.json` or the staff prefs UI to disable sessionStart pulls.
_Avoid_: always-on pull as the silent default, hard-failing the session on network error

**Harness target off**:
`targets_enabled.<target>=false` (e.g. `targets_enabled.pi=false`) skips that Target during hybrid sync.
_Avoid_: uninstalling the Target, deleting Fan-out trees to "turn off" a host

**MCP coexistence**:
Where hosts support it, prefer org MCP `skills_*` tools (and command/agent twins) for progressive disclosure; filesystem `andrew-*` Fan-out stays dual-run until the MCP exit criterion. If `npx skills` wipes first-party Fan-outs, recover with `agent-sync sync --source hybrid`.
_Avoid_: treating MCP as sole source of truth before exit criterion, relying on `npx skills` to manage `andrew-*` trees

**Pi install paths**:
Pi is a first-class Target: skills under `~/.pi/agent/skills/`, agents at `~/.pi/agent/agents/<name>.md`, hooks via `~/.pi/agent/extensions/` (Pi renamed `hooks/` → `extensions/`). Library commands fan out as skill packages under skills (invoked as `/skill:name`).
_Avoid_: writing into a legacy `~/.pi/agent/hooks/` dir, inventing a separate Pi commands directory
