---
name: commit
version: 0.1.0
description: "Quick conventional commit + optional push. Trigger with \"commit this\", \"make a commit\", \"commit and push\", \"wrap this work\", or when ready to land a focused change without the full /pr-creation workflow. For PR-flavored work use /pr-creation instead."
allowed-tools: Read, Bash, AskUserQuestion
disable-model-invocation: true
argument-hint: "[optional commit message override]"
---

## Overview

Lightweight commit wrapper for the most common workflow: stage → conventional message → commit → optional push. Skips everything that `/pr-creation` does (rebase, changelog, PR open) so it's safe to use for in-progress work on a feature branch.

**Why this skill exists**: usage telemetry shows raw `git add`/`git commit`/`git push` cycles dominate Bash invocations. This wrapper standardizes the conventional-commit message format, surfaces what is about to be committed, and removes the boilerplate of staging + pushing.

## Capability matrix

```
┌─────────────────────────────────────────────────────────────────┐
│                           /commit                               │
├─────────────────────────────────────────────────────────────────┤
│  STANDALONE (always works)                                      │
│  ✓ Show staged + unstaged status                                │
│  ✓ Generate conventional commit message from diff               │
│  ✓ Confirm + commit                                             │
│  ✓ Offer push when on a non-default branch                      │
├─────────────────────────────────────────────────────────────────┤
│  SUPERCHARGED (when these are available)                        │
│  + `/caveman commit` loaded: ultra-terse subjects (≤50 chars,   │
│    body only if "why" is non-obvious)                           │
│  + `gh` CLI: after a push on a feature branch, surfaces the     │
│    "Create PR" URL from the push output                         │
└─────────────────────────────────────────────────────────────────┘
```

For a full PR-grade flow (rebase + changelog + PR body), use `/pr-creation`. This skill is the fast path for "land what I just did".

## Example

**Input**: `/commit` (user has 3 modified files unstaged, on branch `feat/invite-tokens`)

**What happens**:

1. `git status --short` shows `M src/invites/token.ts`, `M src/invites/service.ts`, `?? tests/invites/token.test.ts`
2. Nothing staged → AskUserQuestion: "Stage all changes (`git add -A`)?" → user picks Yes
3. Read `git diff --cached` → infer type `feat`, scope `invites`
4. Propose: `feat(invites): add invite token hashing + 7-day TTL`
5. AskUserQuestion confirm/edit → user accepts
6. `git commit -m "..."` → success
7. Branch is not main/master → AskUserQuestion: "Push to origin?" → user picks Yes
8. `git push -u origin feat/invite-tokens` → done; print remote URL if surfaced

## How it works

1. Run `git status --short` and `git rev-parse --abbrev-ref HEAD` to capture state and current branch.
2. If nothing is staged and user did not say "stage all" in the argument, AskUserQuestion whether to run `git add -A`.
3. Read the staged diff with `git diff --cached` (limit to first ~500 lines; large diffs get summarized by file).
4. Infer commit metadata:
   - `type` — feat / fix / refactor / docs / test / chore / perf / ci / style
   - `scope` — top-level changed dir (e.g. `invites`, `auth`, `api`) when one dominates
   - `subject` — imperative present-tense summary
   - `body` — only when the "why" is non-obvious (e.g. bug fix referencing root cause)
5. If `/caveman commit` mode is active, switch to caveman style (≤50 char subject, body only if necessary).
6. If the user passed a message override in `$ARGUMENTS`, use it verbatim and skip generation.
7. Show proposed message; AskUserQuestion to confirm / edit / cancel.
8. Run `git commit -m "$message"` (heredoc for multi-line bodies). On failure (e.g. pre-commit hook), surface stderr and stop — do NOT --amend.
9. Detect current branch. If not `main` / `master` / `develop` / `trunk`, AskUserQuestion whether to push. Run `git push` (or `git push -u origin <branch>` on first push).
10. Print final status: commit SHA, branch, push result (and PR URL if surfaced by `git push` output).

## Gotchas

- Pre-commit hook failure means the commit did NOT happen. Never `--amend` to recover — fix the hook complaint, re-stage, and run `/commit` again to create a NEW commit.
- `git add -A` will pick up `.env` / credentials if they are not in `.gitignore`. The skill skips files matching `*.env`, `*.pem`, `*credentials*`, `*secret*` and warns the user.
- Generated subjects are inferred from diffs, not from intent. Always review the proposed message — a refactor that looks like a feature will be mislabeled.
- If the working tree contains commits ahead of remote AND uncommitted changes, the skill commits only what is staged. Stash or commit the rest separately.
- Do not use this skill to land work that needs a changelog, PR, or rebase — use `/pr-creation` instead, which handles all three.
