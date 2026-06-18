---
name: code-reduction
version: 0.1.0
description: >-
  Dead-code elimination, dependency cleanup, and deduplication via a quick tool scan (knip/depcheck/vulture) or deep parallel-agent analysis. Use when shrinking the codebase or consolidating duplicate implementations before a refactor.
allowed-tools: Read, Write, Edit, Bash, Glob, Grep
argument-hint: "<scope: directory, module, or feature area> [mode: quick|deep|all]"
---

## When to use

Two modes: (1) Quick scan via knip/depcheck/vulture; (2) Deep analysis via parallel Explore agents. Trigger phrases: "find dead code", "shrink the codebase", "remove unused dependencies", "deduplicate this module", "clean up before refactor".

## Overview

Systematic code reduction: discovers redundancy, removes dead code, identifies duplicates, finds gaps, and consolidates. Two modes available:

- **Quick scan** (default for targeted cleanup) — runs knip, depcheck, or vulture for fast tool-driven dead code detection
- **Deep analysis** (default for broad scope) — spawns parallel Explore agents for structural analysis, deduplication, and gap detection
- **All** — runs quick scan first, then escalates to deep analysis for anything the tools missed

## Capability matrix

```
┌─────────────────────────────────────────────────────────────────┐
│                       /code-reduction                           │
├─────────────────────────────────────────────────────────────────┤
│  STANDALONE (always works)                                      │
│  ✓ Manual Grep/Glob redundancy scan                             │
│  ✓ Structural duplicate detection via parallel Explore agents   │
│  ✓ Gap detection (missing exports, orphaned modules)            │
│  ✓ Consolidation plan with edit list                            │
├─────────────────────────────────────────────────────────────────┤
│  SUPERCHARGED (when these are available)                        │
│  + knip (TS): unused exports, files, deps in one pass           │
│  + ts-prune: secondary unused-export check                      │
│  + depcheck: unused npm dependencies                            │
│  + vulture (Python): dead code detection                        │
│  + pip-extra-reqs: unused Python dependencies                   │
│  + staticcheck / go vet (Go): dead code and unused identifiers  │
│  + LSP (typescript-lsp / pyright-lsp): safe rename + references │
└─────────────────────────────────────────────────────────────────┘
```

Without the static analysis tools, the skill skips Step 0 quick scan and goes straight to agent-driven deep analysis.

## Decision rules from the SWE classics

This skill applies decision rules distilled from the following books (vendored under `references/books/`):

- **Refactoring** (Fowler) — [mini](../../../pixl-crew-core/references/books/refactoring/refactoring.mini.md): Extract Function, Inline Function, Move Function/Field, Replace Conditional with Polymorphism. Use when consolidating duplicates or collapsing thin layers.
- **Refactoring Guru** — [mini](../../../pixl-crew-core/references/books/refactoring-guru/refactoring-guru.mini.md): modern companion catalog with broader smell coverage. Use as cross-reference when Fowler's catalog lacks a name for the target pattern.

Cite the named refactoring + book when proposing a structural change so the user can verify the safe-rewrite path.

## Example

**Input**: `/code-reduction src/ quick`

**What happens**:

1. Detects TypeScript from `package.json`, finds `knip`, `depcheck` available
2. Runs `npx knip --reporter compact` — surfaces unused files, exports, deps
3. Triages results: truly unused → delete; dynamically used / framework magic → keep + ignore
4. Applies deletions (if user approves), runs typecheck to verify
5. Reports LOC removed + deps trimmed

**Output**:

```text
Code Reduction — src/ (quick mode)

UNUSED FILES (delete) — 7 files, -342 LOC
  src/utils/legacy-formatter.ts
  src/components/OldButton.tsx
  src/hooks/useDeprecatedAuth.ts
  ...

UNUSED EXPORTS — 23 exports across 11 files
  src/lib/api.ts: `fetchUserLegacy`, `parseOldFormat`
  src/types/index.ts: `LegacyConfig`, `DeprecatedProps`
  ...

UNUSED DEPS (depcheck) — 4 packages
  moment, classnames, prop-types, react-helmet

KEPT (flagged but used dynamically)
  src/routes/[locale].tsx — Next.js file convention (added to knip.json ignore)

Applied: -342 LOC, 4 deps removed (saves 1.2MB bundle). tsc: clean.
```

## Required References

Before starting, read `references/methodology/refactor-planning.md` for safe refactoring strategies and dependency analysis patterns.

## Step 0: Quick Scan (Tool-Driven)

Detect project type and run the appropriate static analysis tools:

| Signal | Tools available |
|---|---|
| `package.json` with TypeScript | knip, ts-prune, depcheck |
| `package.json` without TypeScript | depcheck |
| `pyproject.toml` | vulture, pip-extra-reqs |
| `go.mod` | `go vet`, `staticcheck` |

```bash
# TypeScript/Node.js — knip is the best all-in-one tool
npx knip --reporter compact    # Finds unused files, exports, deps, types

# Fallbacks if knip is not available:
npx depcheck                   # Unused deps only

# Python
pip install vulture && vulture src/

# Go
go vet ./...
staticcheck ./...
```

### Triage quick scan results

| Category | Action |
|---|---|
| **Truly unused** — no references anywhere | Delete |
| **Dynamically used** — referenced via string, reflection, or config | Keep, add to knip ignore |
| **Test-only export** — only used in tests | Keep (but check if test is dead too) |
| **Plugin/framework magic** — used by framework convention | Keep, add to knip ignore |

If mode is `quick`, apply deletions (Step 4) and report (Step 5). If mode is `deep` or `all`, continue to Step 1.

## Step 1: Discovery (Parallel — 3 Agents)

Spawn 3 Explore agents to scan in parallel:

1. **Agent A**: Map exports, imports, and dependency graph
2. **Agent B**: Identify code patterns and potential duplicates
3. **Agent C**: Catalog test coverage and unused test helpers

## Step 2: Dead Code Detection

Analyze the dependency graph to find:

1. Unused exports (exported but never imported)
2. Unreachable branches (conditions that are always true/false)
3. Unused variables and parameters
4. Orphaned files (not imported by anything)
5. Commented-out code blocks

> **Tooling:** Consider using `ts-prune` (`npx ts-prune`) to detect unused TypeScript exports automatically.

## Step 3: Duplicate Detection

Find code that appears in multiple places:

1. Exact duplicates (copy-paste)
2. Near-duplicates (same structure, different names)
3. Pattern duplicates (same logic, different data types)
4. Suggest consolidation into shared utilities

## Step 4: Gap Analysis

Identify what's missing:

1. Missing tests for critical paths
2. Missing error handling
3. Inconsistent patterns (some modules follow a pattern, others don't)
4. Missing type definitions

## Step 5: Remediate

Apply fixes with safety:

1. Remove confirmed dead code
2. Extract duplicates into shared utilities
3. Run tests after each change to ensure no regressions
4. Report remaining items that need manual review

## Step 6: Verify

```bash
# TypeScript
npx tsc --noEmit               # Still compiles
npm test                        # Tests still pass

# Python
python -m pytest tests/         # Tests still pass

# Re-run analysis to confirm reduction
npx knip                        # Count decreased
```

## Step 7: Report

```markdown
## Cleanup Summary
- Dead files removed: N
- Dead exports removed: N
- Unused deps removed: N (saved ~X MB)
- Dead code lines removed: N
- Duplicates consolidated: N
- Remaining items (kept with justification): N
```
