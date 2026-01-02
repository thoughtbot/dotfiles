#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="$HOME/.claude"

echo "Installing Claude Code configuration files..."

# Create target directories
mkdir -p "$TARGET_DIR/commands"
mkdir -p "$TARGET_DIR/agents"

# Copy commands
if [ -d "$SCRIPT_DIR/commands" ]; then
    cp -v "$SCRIPT_DIR/commands/"*.md "$TARGET_DIR/commands/" 2>/dev/null || true
fi

# Copy agents
if [ -d "$SCRIPT_DIR/agents" ]; then
    cp -v "$SCRIPT_DIR/agents/"*.md "$TARGET_DIR/agents/" 2>/dev/null || true
fi

# Copy settings.local.json
if [ -f "$SCRIPT_DIR/settings.local.json" ]; then
    cp -v "$SCRIPT_DIR/settings.local.json" "$TARGET_DIR/"
fi

echo "Done! Files installed to $TARGET_DIR"
