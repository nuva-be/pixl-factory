---
name: self-review-fix-loop
version: 0.1.0
description: >-
  Multi-agent self-review and remediation for open changes: parallel reviewers consolidate findings, then parallel fixers iterate until no gaps remain (max 10 iterations). Use when a feature is functionally done but needs a quality pass before review.
allowed-tools: Read, Write, Edit, Bash, Glob, Grep
argument-hint: "<scope: git diff, feature name, or file list>"
context: fork
---

## When to use

Trigger phrases: "self-review this", "polish this before I ship", "find the gaps", "review and fix", "keep iterating until it's good".

## Overview

This skill runs an iterative review-and-fix loop over code changes. Each iteration has two waves of parallel agents: **3 reviewers** that find issues, then **3 fixers** that resolve them. The coordinator consolidates findings between waves and partitions work across fixers to avoid conflicts.

The loop continues until reviewers find no meaningful issues or the iteration cap (10) is reached.

**Why self-review works**: Automated self-review catches 80%+ of issues before human review because it applies multiple specialized lenses (correctness, security, maintainability) in parallel — something a single reviewer rarely does consistently.

## Capability matrix

```
┌─────────────────────────────────────────────────────────────────┐
│                    /self-review-fix-loop                        │
├─────────────────────────────────────────────────────────────────┤
│  STANDALONE (always works)                                      │
│  ✓ Reads working-tree diff (staged + unstaged)                  │
│  ✓ 3 parallel reviewer agents + 3 parallel fixer agents         │
│  ✓ Confidence-scored findings, P0–P3 severity                   │
│  ✓ File-partition assignment so fixers don't conflict           │
│  ✓ Iteration cap (10) prevents runaway loops                    │
├─────────────────────────────────────────────────────────────────┤
│  SUPERCHARGED (when these are available)                        │
│  + pytest / vitest / jest: verification step runs the suite     │
│  + LSP (typescript-lsp, pyright-lsp): refactor safety           │
│  + pixl knowledge: faster file lookup for partitioning          │
│  + Sentry MCP: enrich findings with prod error frequency        │
└─────────────────────────────────────────────────────────────────┘
```

Without LSP, fixers fall back to Grep-based rename verification. Without a test runner, the verify step is skipped (warning emitted). Without Sentry, findings lack prod-frequency annotation.

## Decision rules from the SWE classics

This skill applies decision rules distilled from the following books (vendored under `references/books/`):

- **Clean Code** (Martin) — [mini](../../../pixl-crew-core/references/books/clean-code/clean-code.mini.md): readability, naming, function size, and comment hygiene. Use as the lens for reviewer C (tests/maintainability).
- **Refactoring** (Fowler) — [mini](../../../pixl-crew-core/references/books/refactoring/refactoring.mini.md): code smell taxonomy + named refactorings. Use as the lens for fixer agents proposing safe structural changes.

Cite the book + smell name in findings so reviewers and fixers share vocabulary across iterations.

## Example

**Input**: `/self-review-fix-loop` (with uncommitted changes for a new "team invites" feature)

**What happens**:

1. Iteration 1 — 3 reviewers scan the diff in parallel (A: requirements, B: regressions/security, C: tests/maintainability)
2. Coordinator merges findings into `.context/review-findings.json` with normalized severity
3. 3 fixer agents partition the backlog by file ownership, apply fixes in parallel, run tests
4. Final verification: tsc + test suite. New P0/P1 issues feed iteration 2. Loop ends when no findings remain or 10 iterations hit.

**Output** (iteration trace):

```text
Iter 1: Found 7 issues (2 P0, 3 P1, 2 P2)
  P0: Missing auth guard on POST /api/invites
  P0: Invite token not hashed before storage
  P1: No rate-limit on invite endpoint
  ...
Iter 1 fixes: 7/7 applied. Tests: 142 pass, 0 fail.

Iter 2: Found 1 new issue (P2 — extracted helper has no test)
Iter 2 fixes: 1/1 applied. Tests: 145 pass, 0 fail.

Iter 3: No findings at >P2. Loop ends.
Total: 8 issues resolved across 2 iterations.
```

## Quick Start

1. **Scope**: use the git diff (staged + unstaged) by default, or the supplied feature/file list.
2. **Review wave**: spawn 3 read-only Explore agents in parallel (A=requirements, B=regressions/security, C=tests/maintainability). Each returns P0-P3 findings with evidence.
3. **Consolidate**: merge + dedupe + normalize severity → write `.context/review-findings.json`. If empty, end the loop.
4. **Fix wave**: partition findings by file ownership across 3 general-purpose fixer agents in parallel. Each runs tests after edits.
5. **Verify**: run full test suite + typecheck + lint. New P0/P1 → another iteration (max 10). Otherwise end.

For the findings packet schema, severity table, iteration controls, and detailed reviewer/fixer contracts, read [`reference.md`](./reference.md).

## See also

- [`reference.md`](./reference.md) — findings packet format, severity rubric, iteration control
- `references/orchestration/context-packet.md` — packet standard used for `review-findings.json`
- `/cto-review` — run after the loop stabilizes for architectural assessment
- `/cartographer` — decompose the resulting diff into semantic feature clusters
