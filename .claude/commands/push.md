---
allowed-tools: Bash(git status:*), Bash(git push:*), Bash(git branch:*), Bash(gh pr create:*), Bash(git log:*), Bash(git diff:*)
description: Create a pull request
---

## Context

- Current branch: !`git branch --show-current`
- Git status: !`git status`
- Commits on this branch (not on main): !`git log main..HEAD --oneline`
- Full diff from main: !`git diff main...HEAD`

## Your task

Based on the above changes, create a pull request against main. If the branch hasn't been pushed yet, push it first with -u flag.
