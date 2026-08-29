#!/usr/bin/env bash
# pull-if-stale.sh — hard timeout 3s on pull; never block session start
set -euo pipefail
BIN="${AGENT_SYNC_BIN:-$HOME/.local/bin/agent-sync}"
DOTFILES_DIR="${DOTFILES_DIR:-$HOME/dotfiles}"
export DOTFILES_DIR
timeout 3s "$BIN" pull --if-stale || true
"$BIN" sync --source hybrid || true
