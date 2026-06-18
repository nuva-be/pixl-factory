# Self-Review Fix Loop — Reference

Full procedure for `/self-review-fix-loop`. Read this when SKILL.md instructs you to, or when you need depth on a specific reviewer, the findings packet format, or iteration control.

## Step 1: Scope

Determine the review scope:

- If there is a `git diff` (staged + unstaged), that is the default scope
- Otherwise, use the provided feature scope or file list
- Identify the changed files, their purpose, and the intent of the changes

## Step 2: Review (Parallel — 3 Agents)

Spawn **3 reviewer agents** (Explore type, read-only) in parallel:

- **Reviewer A**: Requirement coverage and behavioral correctness
- **Reviewer B**: Regressions, edge cases, error handling, security/privacy, performance
- **Reviewer C**: Tests, maintainability, instruction/policy compliance

Each reviewer returns findings with severity (P0-P3), evidence, and fix direction.

## Step 3: Consolidate

The coordinator merges, deduplicates, and normalizes severity across all reviewer outputs:

1. Merge findings from all reviewers
2. Deduplicate (same file + same issue = one finding)
3. Normalize severity across reviewers
4. Produce a single prioritized backlog
5. **If the backlog is empty, the loop ends**

### Findings Packet Format

Write consolidated findings to `.context/review-findings.json` using the context packet standard (see `references/orchestration/context-packet.md`):

```json
{
  "type": "review",
  "version": "1.0",
  "metadata": {
    "skill": "self-review-fix-loop",
    "project": "PROJECT",
    "created_at": "TIMESTAMP"
  },
  "payload": {
    "findings": [
      {
        "severity": "P0",
        "file": "src/api/users.ts",
        "description": "Missing auth check on DELETE endpoint",
        "fix_direction": "Add requirePermission guard before handler",
        "evidence": "Line 42: router.delete('/users/:id', handler) has no guard"
      }
    ]
  }
}
```

## Step 4: Fix (Parallel — 3 Agents)

Read the consolidated findings from `.context/review-findings.json`. Partition the backlog into **3 non-overlapping packets** by file ownership. Spawn **3 fixer agents** (general-purpose) in parallel. Each fixer:

1. Implements fixes only within its assigned file scope
2. Runs relevant tests after each fix
3. Reports results (fixed, skipped, blocked)

## Step 5: Verify

Archive the findings to `.context/review-findings-final.json` for audit trail.

Integrate fixer outcomes, resolve conflicts, and run final verification:

- Run full test suite
- Run typecheck and linter
- If verification fails, create new P0/P1 findings and continue the loop
- If all passes, the loop ends

## Iteration Control

| Parameter          | Default | Description                                            |
| ------------------ | ------- | ------------------------------------------------------ |
| `max_iterations`   | 10      | Hard cap on review-fix cycles                          |
| `stop_at_severity` | P2      | Stop when no findings at this severity or above remain |

## Severity Levels

| Level | Meaning                              | Action                             |
| ----- | ------------------------------------ | ---------------------------------- |
| P0    | Blocking correctness or safety issue | Always fix                         |
| P1    | High-impact bug or regression risk   | Always fix                         |
| P2    | Moderate issue                       | Fix unless explicitly out of scope |
| P3    | Low-impact polish                    | Fix when low-cost and safe         |

## Related Skills

- **`/cto-review`** — Run after the review-fix loop stabilizes to get an architectural assessment of remaining complexity
- **`/cartographer`** — Run on the final diff to decompose changes into semantic feature clusters for PR review
