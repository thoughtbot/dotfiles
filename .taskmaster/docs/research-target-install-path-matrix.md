# Research: Target Install Path Matrix
<!-- Task Master: agent-sync / Task #2 -->

**Status:** Complete — 2026-08-17
**Question:** For each v1 Target, what are the correct global directories for skills, commands, agents, and hooks — and which kinds does each Target not support?
**Sources:** First-party docs only (cited per claim). Conflicts with `vercel-labs/skills` noted.

---

## 1. Matrix: Target × Kind → Global Path

> **Legend**
> - Path cells show the **global (user-wide) directory**, relative to `~`.
> - `UNSUPPORTED` = the Target has no native concept for that kind; `verify` should report skip.
> - `(legacy)` = path still works but is deprecated; prefer the skills path for new installs.
> - File pattern shown where applicable: `<name>` is the skill/command/agent name.

| Target | Skills | Commands | Agents | Hooks |
|--------|--------|----------|--------|-------|
| **Claude Code** | `~/.claude/skills/<name>/SKILL.md` | `~/.claude/commands/<name>.md` | `~/.claude/agents/<name>.md` | `~/.claude/settings.json` → `hooks` key |
| **Cursor** | `~/.cursor/skills/<name>/SKILL.md` | `~/.cursor/commands/<name>.md` (legacy, deprecated Jul 2026) | `~/.cursor/agents/<name>.md` | `~/.cursor/hooks.json` (file); scripts under `~/.cursor/hooks/` |
| **`.agents`** | `~/.agents/skills/<name>/SKILL.md` | `~/.agents/commands/<name>.md` | `~/.agents/subagents/<name>.md` | `~/.agents/hooks/<name>/HOOK.yaml` + sidecar scripts |
| **OpenCode** | `~/.config/opencode/skills/<name>/SKILL.md` | `~/.config/opencode/commands/<name>.md` | `~/.config/opencode/agents/<name>.md` | UNSUPPORTED (requires JS/TS plugin; no native shell hooks) |
| **Pi** | `~/.pi/agent/skills/<name>/SKILL.md` | as skills under `~/.pi/agent/skills/<name>/` (no native commands dir; invoke via `/skill:name`) | `~/.pi/agent/agents/<name>.md` (loaded by Pi subagent extension) | TypeScript extensions under `~/.pi/agent/extensions/*.ts` + managed registry `.agent-sync-managed.json` (Pi renamed `hooks/` → `extensions/` in 0.84) |

---

## 2. Per-Target Detail

### 2.1 Claude Code

**Source:** [Anthropic — Skills (formerly slash commands)](https://docs.anthropic.com/en/docs/claude-code/slash-commands), [Sub-agents](https://docs.anthropic.com/en/docs/claude-code/sub-agents), [Hooks reference](https://docs.anthropic.com/en/docs/claude-code/hooks), [Settings](https://docs.anthropic.com/en/docs/claude-code/settings)

**User directory:** `~/.claude/`

| Kind | Global path | Format | Notes |
|------|-------------|--------|-------|
| **skills** | `~/.claude/skills/<name>/SKILL.md` | Directory with `SKILL.md` | "Personal skills available in all projects." [skills docs] |
| **commands** | `~/.claude/commands/<name>.md` | Single `.md` file | Commands were merged into skills ("Custom commands have been merged into skills") but `~/.claude/commands/` still works [skills docs]. A skill and command of the same name: skill takes precedence. |
| **agents** | `~/.claude/agents/<name>.md` | Markdown + YAML frontmatter | "User subagents (`~/.claude/agents/`) are personal subagents available in all your projects." [sub-agents docs] |
| **hooks** | `~/.claude/settings.json` → `hooks` key | JSON settings file | "Hooks are defined in JSON settings files." User-scope settings = `~/.claude/settings.json`. Not a directory — hooks merge into the JSON file. [hooks docs] |

**Scopes table from docs:**

| Scope | Location |
|-------|----------|
| User | `~/.claude/` |
| Project | `.claude/` |

---

### 2.2 Cursor

**Source:** [Cursor — Skills](https://cursor.com/docs/skills), [Subagents](https://cursor.com/docs/subagents.md), [Hooks](https://cursor.com/docs/hooks.md)

**User directory:** `~/.cursor/`

| Kind | Global path | Format | Notes |
|------|-------------|--------|-------|
| **skills** | `~/.cursor/skills/<name>/SKILL.md` | Directory with `SKILL.md` | "User-level (global): `~/.cursor/skills/`" [Cursor skills docs]; also `~/.agents/skills/` (shared with agents-cli / npx skills canonical path) |
| **commands** | `~/.cursor/commands/<name>.md` *(legacy)* | Single `.md` file | "As of July 2026 the commands page is gone from cursor.com/docs, and the built-in `/migrate-to-skills` skill converts existing commands. Old `.cursor/commands/` files still load." [learncursor.dev] Agent-sync should target `~/.cursor/skills/` for all new installs; commands fan-out to `~/.cursor/skills/`. |
| **agents** | `~/.cursor/agents/<name>.md` | Markdown + YAML frontmatter | "User subagents: `~/.cursor/agents/`" [Cursor subagents docs]. Cursor CLI (not IDE) currently has a known bug where `~/.cursor/agents/` is not shown in completions — only project-level `agents/` appear. [forum report] |
| **hooks** | `~/.cursor/hooks.json` (config file); scripts at `~/.cursor/hooks/` | JSON config + shell scripts | "For user-level hooks that apply globally, create `~/.cursor/hooks.json`" [Cursor hooks docs]. Hook scripts' relative paths resolve from `~/.cursor/`. |

**Known Cursor bug (symlinks):** Cursor does not follow symlinks during skills discovery. If `~/.cursor/skills/` is a symlink (as `npx skills` creates) or if individual skill directories are symlinks, Cursor will not discover them. Agent-sync must use **direct copies** (not symlinks) for the Cursor target. [Cursor forum: "Missing Global Skills" bug report]

---

### 2.3 `.agents` (agents-cli)

**Source:** [muqsitnawaz/agents-cli AGENTS.md](https://github.com/muqsitnawaz/agents-cli/blob/23dfc023cc6589b756c2b07498de2ed985358c39/AGENTS.md), [docs/00-concepts.md](https://github.com/muqsitnawaz/agents-cli/blob/main/docs/00-concepts.md)

**User directory:** `~/.agents/`

| Kind | Global path | Format | Notes |
|------|-------------|--------|-------|
| **skills** | `~/.agents/skills/<name>/SKILL.md` | Directory with `SKILL.md` | "Skills: Knowledge packs (subdirectory per skill)" [concepts doc]. agents-cli syncs this to all managed agents. |
| **commands** | `~/.agents/commands/<name>.md` | Single `.md` file | "Commands: Slash commands (Markdown or TOML)" [concepts doc]. Synced to all managed agents' commands directories. |
| **agents** | `~/.agents/subagents/<name>.md` | Markdown | "Subagents: Subagent workflow definitions" [AGENTS.md]. Note: the kind in agents-cli is named `subagents/`, not `agents/`. |
| **hooks** | `~/.agents/hooks/<scope>/<name>/HOOK.yaml` + sidecar scripts | YAML manifest + shell scripts | "A hook is a bundle directory containing a `HOOK.yaml` manifest plus any sidecar assets." Scopes: `global/` (every project) or `<project>/`. [AGOrcha/dot-agents HOOKS.md] |

---

### 2.4 OpenCode

**Source:** [OpenCode — Config](https://opencode.ai/docs/config), [Agents](https://opencode.ai/docs/agents/), [Commands](https://open-code.ai/en/docs/commands), [Plugins/hooks](https://opencode.ai/v2/docs/build/plugins)

**User directory:** `~/.config/opencode/`

| Kind | Global path | Format | Notes |
|------|-------------|--------|-------|
| **skills** | `~/.config/opencode/skills/<name>/SKILL.md` | Directory with `SKILL.md` | "The `.opencode` and `~/.config/opencode` directories use plural names for subdirectories: `agents/`, `commands/`, `modes/`, `plugins/`, `skills/`, `tools/`, and `themes/`." [config docs] |
| **commands** | `~/.config/opencode/commands/<name>.md` | Markdown file | "You can also define commands using markdown files. Place them in: Global: `~/.config/opencode/commands/`" [OpenCode commands docs] |
| **agents** | `~/.config/opencode/agents/<name>.md` | Markdown + YAML frontmatter | "You can also define agents using markdown files. Place them in: Global: `~/.config/opencode/agents/`" [OpenCode agents docs] |
| **hooks** | **UNSUPPORTED natively** | — | OpenCode hooks require a JS/TS plugin. Native shell hooks like Claude Code's or Cursor's do not exist. Third-party plugin `opencode-yaml-hooks` provides YAML hooks at `~/.config/opencode/hook/hooks.yaml`, but this requires a plugin registration step. `verify` should report skip for OpenCode hooks. |

---

### 2.5 Pi (pi-coding-agent by earendil-works)

**Source:** [earendil-works/pi — skills.md](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/docs/skills.md), [README](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/README.md), [pi.dev/docs/latest/skills](https://pi.dev/docs/latest/skills)

**User directory:** `~/.pi/agent/`

| Kind | Global path | Format | Notes |
|------|-------------|--------|-------|
| **skills** | `~/.pi/agent/skills/<name>/SKILL.md` | Directory with `SKILL.md` | "Pi loads skills from: Global: `~/.pi/agent/skills/`, `~/.agents/skills/`" [Pi skills docs]. Pi also reads from `~/.agents/skills/` natively, so agent-sync can write to Pi's own dir or rely on the `.agents` target for shared coverage. |
| **commands** | `~/.pi/agent/skills/<name>/` (as skill packages) | Directory with `SKILL.md` | Pi has no native commands directory. agent-sync fans Library `commands` out as skill packages under `~/.pi/agent/skills/` (invoked as `/skill:name`). |
| **agents** | `~/.pi/agent/agents/<name>.md` | Markdown + YAML frontmatter | Official Pi subagent extension discovers agents from `~/.pi/agent/agents/*.md` (user) and `.pi/agents/*.md` (project). Core Pi does not load these without that extension (or equivalent). agent-sync strips `model` frontmatter for Pi. |
| **hooks** | `~/.pi/agent/extensions/*.ts` | TypeScript extension modules | Pi 0.84 renamed `hooks/` → `extensions/`; a leftover `hooks/` dir triggers a deprecation warning and is not loaded. agent-sync writes thin `.ts` wrappers that `spawn` pack shell scripts, plus `~/.pi/agent/extensions/.agent-sync-managed.json` for ownership (no host JSON merge file). |

---

## 3. Consolidated Supported-Kinds Table

| Target | skills | commands | agents | hooks |
|--------|:------:|:--------:|:------:|:-----:|
| Claude Code | ✓ | ✓ (legacy `.md`) | ✓ | ✓ (JSON key, not dir) |
| Cursor | ✓ | ✓ (legacy; prefer skills) | ✓ | ✓ (JSON file + scripts dir) |
| `.agents` | ✓ | ✓ | ✓ (`subagents/`) | ✓ (YAML bundle) |
| OpenCode | ✓ | ✓ | ✓ | ✗ |
| Pi | ✓ | ✓ (as skills) | ✓ (`agents/*.md` via subagent ext) | ✓ (`extensions/*.ts` + managed registry) |

---

## 4. Conflicts with `vercel-labs/skills` (`npx skills`) Mappings

**Source:** [vercel-labs/skills README](https://github.com/vercel-labs/skills), [installation-methods guide](https://vercel-labs-skills.mintlify.app/guides/installation-methods), [Issue #693 (Claude Code global bug)](https://github.com/vercel-labs/skills/issues/693), [Issue #694 fix (symlink bug)](https://github.com/vercel-labs/skills/issues/694/linked_closing_reference)

### 4.1 Path conflict: `~/.agents/skills/` is dual-purpose

`vercel-labs/skills` uses `~/.agents/skills/` as its **canonical global install location** for all agents:

```
Global install by npx skills:
  ~/.agents/skills/<name>/      # canonical copy
  ~/.claude/skills/<name>/      # symlink → ~/.agents/skills/<name>
  ~/.cursor/skills/<name>/      # symlink → ~/.agents/skills/<name>
```

The `.agents` Target in agent-sync **also** uses `~/.agents/skills/` as its native skills directory (agents-cli format). Files from both sources coexist in `~/.agents/skills/`, differentiated only by skill name. There is no schema collision as long as `agent-sync` Library skill names do not clash with third-party skill names installed by `npx skills`.

**Resolution:** Accept coexistence. agent-sync manages Library-named skills; `npx skills` manages third-party skills. The wayfinder decision is: "Third-party — still `npx skills`."

### 4.2 Symlink conflict: Claude Code and Cursor skills dirs

When `npx skills add -g` runs, it creates symlinks:

- `~/.claude/skills/` → `~/.agents/skills/`
- `~/.cursor/skills/` → `~/.agents/skills/`

If agent-sync then tries to write individual skill directories into `~/.claude/skills/` or `~/.cursor/skills/`, those writes actually land in `~/.agents/skills/` (because the parent is a symlink). This is acceptable for Claude Code (it can read from `~/.claude/skills/` which points to `~/.agents/skills/`), but creates **two problems**:

1. **Cursor symlink bug**: Cursor's skills discovery does not follow symlinks ([forum: "Missing Global Skills"]). Skills installed via symlinks in `~/.cursor/skills/` will not be visible to Cursor. Agent-sync must use **direct copies** for Cursor, or ensure `~/.cursor/skills/` itself is not a symlink before writing.

2. **Path identity**: If `~/.claude/skills/` is a symlink to `~/.agents/skills/`, agent-sync writes to Claude Code target may collide with `.agents` target writes if both write the same skill name. agent-sync should detect and warn when the parent directory is a symlink and coerce to copy mode.

### 4.3 `vercel-labs/skills` agent table (for reference)

From the `npx skills` README:

| Agent | Project-level `skillsDir` | Global `globalSkillsDir` |
|-------|--------------------------|--------------------------|
| Claude Code | `.claude/skills/` | `~/.claude/skills/` |
| Cursor | `.agents/skills/` | `~/.cursor/skills/` |
| OpenCode | `.opencode/skills/` | `~/.config/opencode/skills/` |
| (universal / amp, Codex, etc.) | `.agents/skills/` | `~/.agents/skills/` |

Note that Pi is not listed in `vercel-labs/skills` as a supported agent (as of 2026-08-17). agent-sync must manage Pi's `~/.pi/agent/skills/` independently.

---

## 5. Adapter Skip Rules for `verify`

Based on the matrix, `agent-sync verify` should emit a skip (not an error) for:

| Target | Skip rule |
|--------|-----------|
| OpenCode | `hooks` kind → skip; no native shell hook support |

All other cells in the matrix are supported and should be verified as present.

---

## 6. Notes for agent-sync Adapter Implementation

1. **Hooks are never a plain directory for any Target.** Claude Code hooks live in `~/.claude/settings.json` (JSON key merge); Cursor hooks live in `~/.cursor/hooks.json` (JSON file) with scripts at `~/.cursor/hooks/`; `.agents` hooks are YAML bundle directories. Hook pack fan-out requires distinct merge logic per target — see Task #3 research.

2. **Cursor commands deprecated.** Agent-sync should fan out Library "commands" kind to `~/.cursor/skills/` (not `~/.cursor/commands/`) for the Cursor target, treating commands as skills. This aligns with Cursor's documented direction.

3. **Pi commands → skills; agents + extensions.** Pi has no commands dir: fan Library commands to `~/.pi/agent/skills/`. Agents go to `~/.pi/agent/agents/*.md` (subagent extension convention). Hooks install as TypeScript under `~/.pi/agent/extensions/` (not deprecated `hooks/`), with `.agent-sync-managed.json` tracking ownership.

4. **`.agents` subagents vs agents.** agents-cli uses `~/.agents/subagents/` for agent definitions, but Library uses `agents/` as the kind name. The adapter must map Library `agents/` → `.agents` target `subagents/`.

5. **OpenCode hooks.** Hooks are unsupported. If the Library contains hook packs, fan-out to OpenCode skips them silently and `verify` reports skip (not failure).

6. **Cursor copy mode.** Due to the symlink-discovery bug, agent-sync must use the `copy` install mode (not symlink) for all Cursor target writes under `~/.cursor/skills/` and `~/.cursor/agents/`.
