# agent-sync

`agent-sync` fans out the agent-neutral `library/` into Claude Code, Cursor,
OpenCode, and Pi. Cursor receives copies; other Targets receive symlinks unless
their destination parent is itself a symlink.

Pi notes (Task 10 spike): skills and command-as-skills under
`~/.pi/agent/skills/`; agents under `~/.pi/agent/agents/*.md` (subagent
extension convention); hooks as TypeScript under `~/.pi/agent/extensions/`
(Pi 0.84 renamed `hooks/` → `extensions/`) with
`.agent-sync-managed.json` ownership registry.

## Build

```sh
cd agent-sync
cargo build --release
# setup/common.sh also installs to ~/.local/bin
```

## Usage

```sh
export DOTFILES_DIR=~/dotfiles   # optional; defaults to ~/dotfiles or ~/dotfiles-local
agent-sync list
agent-sync sync --dry-run
agent-sync sync
agent-sync verify
agent-sync doctor
agent-sync doctor --fix
agent-sync migrate --dry-run
agent-sync migrate --write
agent-sync migrate --rollback <backup-id>
```

Sandbox / tests:

```sh
agent-sync sync --root /tmp/agent-sync-sandbox
cargo test
```

Migration is backed up under `.agent-sync-backups/`. `--write` refuses dirty
legacy Target trees unless `--allow-dirty` is supplied. Unsupported
Target/kind combinations are reported and skipped.

Do not run `npx skills remove --all -y` unattended: it scans Target
directories and can delete `andrew-*` fan-outs; recover them with
`agent-sync sync`.

## Legacy home → repo symlinks

Older layouts often symlink `~/.claude/skills` (and Cursor/commands/agents
counterparts) into this clone. Fan-out must land in real `$HOME`
directories. `agent-sync doctor` detects those links; `--fix` replaces them
with empty real directories (then run `sync`). Product `~/.cursor/skills-cursor`
may stay linked into the clone. Installs into the clone outside `library/`
are refused.

## Library layout

See repo `CONTEXT.md` and `.taskmaster/docs/locked-manifest-schema.md`.

## Release workflow

GitHub Actions workflow source lives at `ci/agent-sync-release.yml`. Copy it
to `.github/workflows/agent-sync-release.yml` when the pushing token has the
`workflow` scope (OAuth apps without that scope cannot create workflow files):

```sh
mkdir -p .github/workflows
cp agent-sync/ci/agent-sync-release.yml .github/workflows/agent-sync-release.yml
```

Tag releases as `agent-sync-v*`.
