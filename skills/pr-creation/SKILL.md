---
name: pr-creation
version: 0.1.0
description: "End-to-end pull request workflow: wrap changes, commit, rebase, changelog, create PR. Use to \"create a PR\", \"open a pull request\", \"ship this\", or \"file the PR\" when a feature branch is ready to merge."
allowed-tools: Read, Write, Edit, Bash, Glob, Grep
argument-hint: "<optional: PR title or description>"
disable-model-invocation: true
---

## Capability matrix

```
┌─────────────────────────────────────────────────────────────────┐
│                        /pr-creation                             │
├─────────────────────────────────────────────────────────────────┤
│  STANDALONE (always works)                                      │
│  ✓ Group + stage changes into conventional commits              │
│  ✓ Rebase on base branch (main/master/develop)                  │
│  ✓ Generate changelog entries from the commit set               │
│  ✓ Produce a structured PR description (the *why*, not just     │
│    *what*) ready to paste into any review tool                  │
├─────────────────────────────────────────────────────────────────┤
│  SUPERCHARGED (when these are available)                        │
│  + `gh` CLI (effectively required for autonomous PR opening):   │
│    `gh pr create` opens the PR with title + body in one shot;   │
│    `gh pr view` confirms creation; reviewer suggestions via     │
│    `gh api` repo CODEOWNERS lookup                              │
│  + GitHub MCP: same workflow without shell; lets the skill      │
│    attach reviewers, labels, milestones, and linked issues      │
│  + Linear MCP: auto-link the PR to the originating issue and    │
│    transition state (e.g. In Review)                            │
└─────────────────────────────────────────────────────────────────┘
```

Without `gh` or GitHub MCP, the skill prepares everything locally and instructs the user to open the PR manually. Linear MCP is purely additive for teams that track issues there.

## Overview

Complete PR creation workflow: analyze changes, wrap into conventional commits, rebase on base branch, generate changelog entries, and create the pull request with a structured description.

**Why small, well-described PRs matter**: PRs under 400 lines get reviewed 3x faster and receive higher-quality feedback. A clear description that explains *why* (not just *what*) helps future readers understand intent when git-blaming months later.

## Example

**Input**: `/pr-creation "team invites"` (working tree has 11 modified files, 2 commits ahead of main)

**What happens**:

1. Analyzes diff + recent commits, groups remaining changes into logical units
2. Wraps unstaged work into 2 additional conventional commits: `feat(invites): add invite acceptance flow`, `test(invites): cover expiry edge cases`
3. Rebases on `origin/main` (clean, no conflicts)
4. Detects `CHANGELOG.md` exists → adds a new entry under "Added"
5. Pushes with `-u` and opens the PR via `gh pr create`

**Output**:

```text
PR Created — https://github.com/org/repo/pull/892

TITLE: feat: team invite acceptance flow (#892)

BODY (auto-drafted from commits):
## Summary
- Adds team invite tokens (hashed, 7-day TTL)
- New POST /invites/accept endpoint with rate limit
- Email notification on invite + reminder at 5 days

## Why
Existing team-add flow required admin to create user accounts manually.
This enables self-service invite acceptance while preserving RBAC.

## Test plan
- [x] Unit: invite token hashing + expiry (14 tests)
- [x] Integration: end-to-end accept flow with email mock
- [ ] Manual: verify email rendering on staging

Closes #823

Commits: 4 ahead of main | 312 lines added | 47 removed
```

## Required References

Before starting, read these files:

- `references/standards/commit-conventions.md` — commit message format and conventions
- `references/standards/pr-best-practices.md` — PR description structure and review guidelines

## Step 1: Discovery

1. Analyze `git diff` (staged + unstaged changes)
2. Identify the base branch (main/master/develop)
3. Check for existing changelog file
4. Determine commit convention (conventional commits)
5. Identify related issues or tickets

## Step 2: Wrap Changes

1. Group related changes into logical units
2. Stage files that belong together
3. Ensure no unrelated changes are included
4. Check for files that shouldn't be committed (.env, credentials)

## Step 3: Commit

1. Create conventional commit messages (feat/fix/refactor/docs/test/chore)
2. Include scope and description
3. Add body with details if commit is non-trivial
4. Reference issues in footer

## Step 4: Rebase

1. Fetch latest base branch
2. Rebase feature branch on base
3. Resolve any merge conflicts
4. Verify tests still pass after rebase

## Step 5: Changelog (Conditional)

**Only if changelog file exists.**

1. Generate changelog entries from commits
2. Follow existing changelog format
3. Group by type (Added, Changed, Fixed, Removed)

## Step 6: Create PR

1. Push branch to remote
2. Create PR with structured description:
   - Summary (1-3 bullet points)
   - Changes made
   - Test plan
   - Screenshots (if UI changes)
3. Add labels and reviewers if configured

## Verify

Before declaring the PR shipped, confirm GitHub actually accepted it and the metadata matches the local draft. Surface failures to the user before declaring done.

1. **PR exists on GitHub**:
   ```bash
   PR_URL=$(gh pr view --json url -q .url)
   echo "$PR_URL"
   ```
   Expected: non-empty URL.

2. **Title and body match the local draft** — `gh` silently truncates or rejects bodies with template placeholders:
   ```bash
   gh pr view --json title,body -q '.title, .body' | head -n 30
   ```
   Expected:
   - Title matches the conventional-commits subject (e.g. `feat: team invite acceptance flow`).
   - Body contains `## Summary`, `## Why`, `## Test plan` sections (or whatever template the repo standardizes on).
   - No `{{placeholder}}` strings.

3. **Branch tracking + push succeeded**:
   ```bash
   git rev-parse --abbrev-ref --symbolic-full-name @{u}
   git status -sb | head -n 1
   ```
   Expected: branch tracks a remote; status shows `ahead 0`.

4. **CI started** — confirm GitHub triggered at least one check:
   ```bash
   gh pr checks
   ```
   Expected: at least one check in `queued`/`in_progress`/`completed`. Empty output means no workflow ran (possibly a missing `on: pull_request` trigger).

5. **Links + closes resolved** — if the body references issues:
   ```bash
   gh pr view --json closingIssuesReferences -q '.closingIssuesReferences[].url'
   ```
   Expected: every `Closes #N` resolves to a real issue URL.

6. **PR size sanity**:
   ```bash
   gh pr view --json additions,deletions -q '"+\(.additions) / -\(.deletions)"'
   ```
   Expected: under 400 lines (per `references/standards/pr-best-practices.md`). Larger diffs should be split.

If any step fails, surface the failure and do NOT mark the PR ready. Don't auto-close + reopen the PR to "fix" things — re-push and let GitHub update the existing PR.

## Gotchas

- Always rebase onto the target branch before creating the PR — stale branches cause merge conflicts that block reviewers and CI
- `gh pr create` fails silently if the branch has not been pushed — always push with `-u` first to set the upstream tracking branch
- Use draft PRs (`--draft`) for work-in-progress — this prevents accidental review requests and signals the PR is not yet ready for feedback
- Changelog updates should go in a separate commit from code changes — mixing them makes reverts harder and pollutes the diff reviewers need to focus on
- Never force-push after PR review has started — it destroys review comments and makes it impossible for reviewers to see what changed since their last review
