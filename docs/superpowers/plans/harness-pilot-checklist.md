# Harness client pilot checklist

Manual verification for the closed pilot catalog. Full e2e against a live
harness API requires operator credentials (`AGENT_SYNC_HARNESS_TOKEN` or
`~/.agent-sync/credentials` plus a configured `AGENT_SYNC_HARNESS_URL`); treat
those steps as operator-run, not agent-automated.

## Prerequisites

- [ ] `agent-sync` built (`cargo build --release` in `agent-sync/`)
- [ ] `AGENT_SYNC_HARNESS_URL` + token configured (or skip remote publish/pull)
- [ ] MCP harness tools reachable from the host under test (Claude / Cursor / Pi as applicable)

## Steps

1. **Publish (optional remote)** — `agent-sync publish --items ship-feature,spec-driven,_shared --platform-root <path-to-repo-with-.claude>` or dry-run locally.
2. **Pull + hybrid sync** — `agent-sync pull && agent-sync sync --source hybrid`.
3. **Target installs** — confirm fan-out on Claude, Cursor, and Pi (skills; Pi agents under `~/.pi/agent/agents/`; Pi hooks under `~/.pi/agent/extensions/`).
4. **MCP smoke** — call `skills_list` (and ideally `skills_get` for `ship-feature`) via the org MCP server that exposes harness tools.
5. **Promote + sync-back** — after a human promote on the control plane, run the repo sync-back workflow if applicable.

## Opt-out quick refs

See CONTEXT.md: harness tombstone, pin, live_sync, target off, MCP coexistence.
