---
description: "Full lifecycle: ticket → plan → implement → review → PR. Chains existing skills automatically with gates between stages. Usage: /ship SYSTEM-1354 or /ship 'add retry logic to connector sync'"
alwaysApply: false
---

# Ship

One command, full resolution. Takes a task from description to reviewed PR by
chaining existing skills with gates between stages.

## Arguments

- A Linear ticket ID (e.g. `SYSTEM-1354`) or a free-text description in quotes
- Optional: `--skip-plan` to skip brainstorming/adversary for trivial tasks
- Optional: `--dry-run` to trace the pipeline without implementing

## Pipeline

```
Stage 0 ─── Acquire & Validate ─── flesh out the ticket, understand scope
      │
      ▼
Stage 1 ─── Plan ──────────────── brainstorm → adversary → plan
      │                            (skip with --skip-plan)
      ▼
  ┌─ Gate ── User confirms plan before implementation begins ──┐
  │                                                             │
  ▼                                                             │
Stage 2 ─── Implement & Review ── subagent implements,          │
      │                            swarm-review reviews,         │
      │                            loop until PASS or max        │
      ▼                                                         │
Stage 3 ─── Ship ─────────────── prepare PR, present summary    │
      │                                                         │
      ▼                                                         │
Stage 4 ─── Capture ──────────── session-postmortem             │
                                                                │
  (--dry-run exits here) ◄──────────────────────────────────────┘
```

---

## Stage 0: Acquire & Validate

**If given a Linear ticket ID:**

1. Fetch the ticket via Linear MCP
2. If sparse (empty description, < 50 chars): fetch parent + siblings, synthesize
   intent spec, present for confirmation
3. If the ticket has a `## Plan` section already, use it (skip to Stage 2 gate)
4. Extract acceptance criteria, constraints, scope boundaries

**If given a free-text description:**

1. Use as-is for `TICKET_DESCRIPTION`
2. Assign `TICKET_ID` = "MANUAL"

### Spec & EDR Discovery (required, non-skippable)

Before producing INTENT_SPEC, check for authoritative specs. Specs override priors
and subagent synthesis -- if a spec exists, its content is ground truth.

1. Derive search keywords from the ticket: project name, title nouns, entity names,
   acceptance-criteria terms.
2. Glob for candidates:
   - `docs/internal/specs/*<keyword>*`
   - `docs/internal/edr/*<keyword>*`
   - `docs/internal/analysis/*<keyword>*`
3. For each candidate, invoke the `targeted-file-read` skill with the same keywords
   and the ticket's acceptance criteria as search terms. Never Read specs/EDRs directly
   without that skill -- they are typically 300-600 lines.
4. Collect the returned Extracts into a **Spec Extract** section of INTENT_SPEC.
5. If no specs match, add `Authoritative specs: none found` to INTENT_SPEC. Downstream
   stages will know they are on speculative ground.

Pass the Spec Extract forward verbatim to planning and implementation -- do not
re-summarize or paraphrase it. The citations are load-bearing.

**Output:** `INTENT_SPEC` — what we're building, what done looks like, constraints,
out of scope, **Spec Extract** (or "none found").

Present to user: "Here's what I understand. Anything to adjust before planning?"

---

## Stage 1: Plan

Skip this stage if `--skip-plan` was passed or if the ticket already had a plan.

1. **Brainstorm** (via `superpowers:brainstorming`):
   - Explore approaches, existing patterns in the codebase, adjacent code to reuse
   - Identify which domains are touched (Go API, frontend, schema, DB, infra)

2. **Adversary review** (via `plan-adversary-review`):
   - Three lenses: assumptions, pre-mortem, blast radius
   - If any genuine risks surface, incorporate mitigations

3. **Write plan** (via `superpowers:writing-plans`):
   - Concrete implementation steps with file paths
   - Risks and mitigations from adversary review
   - Acceptance criteria mapped to verification steps

**Output:** Written plan file.

---

## Gate: User Confirms Plan

Present the plan summary:

```
## Ready to implement

Task: [TICKET_ID] [title]
Domains: [Go API, frontend, schema, ...]
Files: ~[N] files across [M] services
Key risks: [top 2 from adversary review]
Plan: [path to plan file]

Proceed to implementation?
```

Wait for user confirmation. If `--dry-run`, stop here and report what would happen.

---

## Stage 2: Implement & Review

This stage runs the full `/implement-and-review-loop` with the context gathered
in Stages 0-1:

1. **Resolve Spectacles context** for the domains identified in Stage 1
2. **Loop** (max 3 iterations by default):
   - Dispatch implementation subagent with ticket + plan + MVC + prior feedback
   - Run quality gates (lint, typecheck, test, Spectacles validate)
   - Run swarm-review against the diff
   - Parse review: PASS → exit, NEEDS WORK → feed back, iterate
3. **On exit:**
   - If PASS: proceed to Stage 3
   - If max iterations with remaining issues: present to user with options:
     - Fix manually and re-run review only
     - Proceed with known issues
     - Abort

**Output:** Clean branch with committed changes, review artifact at `~/.agent/reviews/`.

---

## Stage 3: Ship

1. Run `/prepare-pr` to create the pull request
2. Present summary:

```
## Shipped

Ticket: [TICKET_ID]
Branch: [branch-name]
PR: [PR URL]
Iterations: [N]
Review: PASS

Changes:
[summary of what was implemented]

Remaining (if any):
[deferred systemic findings from review]
```

---

## Stage 4: Capture

Suggest running `session-postmortem` to capture what worked and what didn't.
This feeds the self-improvement loop.

---

## Recovery

| Situation | Action |
|-----------|--------|
| Stage 0 fails (can't fetch ticket) | Ask user for description, continue as free-text |
| Stage 1 plan is rejected by user | Revise plan based on feedback, re-present |
| Stage 2 hits max iterations | Present remaining issues, offer manual fix path |
| Stage 3 PR creation fails | Commit is local — user can push manually |
| Any stage: user says "stop" | Stop cleanly, report what's done so far |

## When NOT to Use

- **Trivial fixes** (typo, one-line change): just do it directly
- **Exploration / research**: use `/investigate` or skills directly
- **Pure review** (no implementation): use `/review-fix-loop`
- **Schema-only changes**: use `psgen-workflow` directly
