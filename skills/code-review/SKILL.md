---
name: code-review
version: 0.1.0
description: "Multi-agent PR review with confidence scoring and AUTO-FIX/ASK classification — quick (3 reviewers) or full (8 specialists incl. red team). Use with a PR number or branch, \"review this PR\", \"audit before merge\", or \"is this safe to ship\"."
allowed-tools: Read, Bash, Glob, Grep, Agent
argument-hint: "[PR number or branch name] [--full | --quick] [--post] [--auto-fix]"
---

## Overview

Structured multi-agent code review for pull requests. Up to 8 parallel specialist reviewers analyze the diff through specialized lenses, findings are deduplicated, scored for confidence, and classified as AUTO-FIX (safe to apply immediately) or ASK (requires user decision).

**Review modes**:
- **Quick** (default, `--quick`): 3 core reviewers (correctness, security, conventions) — for routine PRs <200 lines
- **Full** (`--full`): 8 specialist reviewers — for critical changes, large PRs, or changes touching auth/payments/data

**How this differs from other review skills**:
- `/self-review-fix-loop` — reviews AND fixes working tree changes. This skill is read-only (unless `--auto-fix`).
- `/cto-review` — architectural critique of the full branch state. This skill focuses on the PR diff.
- `/cross-review` — runs two different LLM models independently. This skill uses multiple specialist lenses on the same model.

## Setup

Read `config.json` in this skill directory (if it exists):
- `confidence_threshold` — minimum confidence to include a finding; default 70
- `post_comments` — auto-post findings as PR comments; default false
- `reviewers` — review dimensions to run; default ["correctness", "security", "conventions"]
- `auto_fix` — automatically apply low-risk fixes; default false
- `review_mode` — "quick" (3 core reviewers) or "full" (8 specialists); default "quick"

If `post_comments` is false (default), output findings as a report only. Argument-level flags override config defaults.

## Capability matrix

```
┌─────────────────────────────────────────────────────────────────┐
│                       /code-review                              │
├─────────────────────────────────────────────────────────────────┤
│  STANDALONE (always works)                                      │
│  ✓ Paste a diff or point to a branch                            │
│  ✓ Parallel reviewers: correctness, security, conventions       │
│  ✓ Confidence scoring per finding                               │
│  ✓ AUTO-FIX / ASK classification                                │
│  ✓ Report output (markdown table)                               │
├─────────────────────────────────────────────────────────────────┤
│  SUPERCHARGED (when these are available)                        │
│  + gh CLI: fetch PR diff + CI status from PR number             │
│  + git remote: fetch base ref automatically                     │
│  + --post flag: post findings as inline PR comments via gh      │
│  + --auto-fix: apply low-risk fixes directly to working tree    │
│  + --full: 8 specialist reviewers (incl. red team) for high-risk│
│  + Sentry MCP: enrich findings with prod error frequency        │
└─────────────────────────────────────────────────────────────────┘
```

If `gh` is not installed, fall back to local `git diff` and skip PR-context enrichment. If no Sentry MCP, skip the prod-frequency annotation. Output is always a self-contained report.

## Decision rules from the SWE classics

This skill applies decision rules distilled from the following books (vendored under `references/books/`):

- **Clean Code** (Martin) — [mini](../../../pixl-crew-core/references/books/clean-code/clean-code.mini.md): naming, function size, single responsibility, comment hygiene. Use when the conventions reviewer flags readability or naming concerns.
- **Refactoring** (Fowler) — [mini](../../../pixl-crew-core/references/books/refactoring/refactoring.mini.md): code smell taxonomy (Long Method, Feature Envy, Data Clump, Shotgun Surgery) and refactoring catalog. Use when the maintainability reviewer recommends a structural change.

When findings overlap, cite the book + smell name in the report so reviewers can verify the source.

## Example

**Input**: `/code-review 1234 --full`

**What happens**:

1. `gh pr diff 1234` pulls the diff and PR description for context
2. 8 specialist agents (correctness, security, conventions, performance, API, migrations, maintainability, red-team) review the diff in parallel
3. Findings are deduplicated, confidence-boosted on cross-reviewer agreement, filtered by threshold (70)
4. Findings classified AUTO-FIX vs ASK and grouped by severity

**Output**:

```text
Code Review: PR #1234 — "Add user profile endpoints"
Reviewers: 8 | Findings: 6 (4 filtered)

CRITICAL — ASK (1)
  [95%] src/api/users.ts:42 — Missing auth check on DELETE /users/:id
  Reviewers: B (Security), H (Red Team) — consensus

IMPORTANT — ASK (3)
  [88%] src/models/user.ts:15 — Password field included in toJSON()
  [85%] src/api/users.ts:67 — Unbounded query without pagination
  [82%] prisma/migrations/0043.sql:12 — NOT NULL without default on populated table

AUTO-FIX (suggestions): [92%] import order, [88%] missing await
```

**Or**: `/code-review feat/profiles --quick --post` runs 3 reviewers and posts findings as inline PR comments via `gh pr review`.

## Quick Start

1. **Get the diff**: PR number → `gh pr diff <number>`; branch → `git diff main...<branch>`; nothing → `git diff main...HEAD`.
2. **Spawn reviewers in parallel** via the Agent tool. Quick mode = A/B/C (correctness, security, conventions). Full mode = A–H (adds performance, API contracts, migrations, maintainability, red team). Each reviewer returns JSON findings with `severity`, `confidence`, `fix_class`.
3. **Consolidate**: dedupe by file+line, boost cross-reviewer agreement +10, filter <70 confidence, sort Critical→Important→Minor, group AUTO-FIX vs ASK.
4. **Auto-fix (if `--auto-fix`)**: apply AUTO-FIX findings, re-run lint/typecheck, report what changed.
5. **Output**: structured report with file:line, confidence %, reviewer attribution, and suggested fix. If `--post`, push to PR via `gh pr review`.

For full reviewer checklists (A through H), finding schema, consolidation algorithm, output template, confidence scoring boosters, and gotchas, read [`reference.md`](./reference.md).

## Verify

This skill is read-only by default. The Verify step only runs when `--auto-fix` was passed (the skill actually mutated files). Surface failures to the user before declaring done:

1. **Saved the consolidated report** — confirm the markdown report was emitted (and posted, if `--post`):
   ```bash
   test -f .claude/reports/code-review-<pr>.md
   gh pr view <pr> --comments | grep -c "Code Review"   # when --post
   ```
   Expected: report file exists; `gh` comment count incremented (when `--post`).

2. **AUTO-FIX entries actually landed** — re-grep the diff for each finding that was classified `AUTO-FIX`. For each `file:line` reported:
   ```bash
   git diff -- <file> | grep -nE '<expected fix pattern>'
   ```
   Expected: every AUTO-FIX finding maps to a corresponding diff hunk. Any finding without a matching hunk indicates the fix did NOT land — re-run the fix.

3. **Lint + type check + tests pass on the modified tree**:
   ```bash
   bun run lint && bunx tsc --noEmit && bun test --quiet
   # OR for Python projects:
   uv run ruff check . && uv run pyright && uv run pytest --quiet
   ```
   Expected: 0 errors. Auto-fixes that break the build must be reverted before declaring complete.

4. **Re-review delta** — re-run the consolidated review in dry-run mode and confirm previously-AUTO-FIX findings no longer appear:
   ```bash
   /code-review <pr> --quick
   ```
   Expected: the AUTO-FIX findings from the first pass are gone. Remaining findings should be ASK-classified only.

If any step fails, surface the failure and do NOT mark the review complete. For ASK findings, the user must adjudicate — do not auto-resolve them.

## See also

- [`reference.md`](./reference.md) — per-reviewer checklists, consolidation rules, confidence scoring, data logging
- `references/standards/code-review.md` — confidence scoring standard + known anti-patterns
- `/self-review-fix-loop` — find-then-fix workflow on working tree
- `/cto-review` — architectural critique (full branch, not just diff)
- `/cross-review` — multi-model consensus for critical changes
