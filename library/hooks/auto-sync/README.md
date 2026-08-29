# auto-sync hook pack

Session-start hooks for Cursor, Claude Code, and Pi that keep the harness cache warm.

1. `agent-sync pull --if-stale` with a hard 3s timeout (soft-fail)
2. `agent-sync sync --source hybrid`

Opt out: set `live_sync.mode=off` in `~/.agent-sync/preferences.json`.

Managed entries are tagged `agent-sync:auto-sync:…` (`_as` / Pi managed registry).
