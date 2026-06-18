---
name: changelog
version: 0.1.0
description: "Generate release notes from conventional commits - parses commits since the last git tag, groups by type, and writes a markdown entry. Use to \"generate release notes\", \"update CHANGELOG.md\", \"what changed since last tag\", or \"prepare a release\"."
allowed-tools: Read, Write, Bash, Glob, Grep
argument-hint: "[--dry-run] [--since <tag>] [--output CHANGELOG.md]"
---

## Capability matrix

```
┌─────────────────────────────────────────────────────────────────┐
│                         /changelog                              │
├─────────────────────────────────────────────────────────────────┤
│  STANDALONE (always works)                                      │
│  ✓ Parse conventional commits since latest tag (or `--since`)   │
│  ✓ Group by type (feat/fix/refactor/...), flag breaking changes │
│  ✓ Write or dry-run CHANGELOG.md entry                          │
├─────────────────────────────────────────────────────────────────┤
│  SUPERCHARGED (when these are available)                        │
│  + GitHub MCP / `gh` CLI: enrich entries with PR titles,        │
│    numbers, and authors; pull PR labels for category overrides; │
│    backfill changelog when commits aren't conventional          │
│  + Linear MCP: resolve issue references (e.g. `PIX-123`) into   │
│    canonical titles so changelog lines read as user-facing      │
└─────────────────────────────────────────────────────────────────┘
```

The skill always produces a usable changelog from local git history; gh and Linear MCP add human-readable PR/issue context that raw commit subjects often lack.

## Overview

Parses conventional commits since the last git tag and generates a structured changelog entry. Supports dry-run mode for preview without writing.

## Step 1: Determine Range

1. Find the latest tag: `git describe --tags --abbrev=0 2>/dev/null`
2. If `--since <tag>` is provided, use that instead
3. If no tags exist, use all commits on the current branch

## Step 2: Parse Commits

Run `git log <tag>..HEAD --pretty=format:"%H|%s|%an" --no-merges` and parse each line:

- Extract type from conventional commit prefix: `feat`, `fix`, `refactor`, `chore`, `docs`, `test`, `perf`, `ci`, `style`
- Extract optional scope: `feat(crew): ...` → scope = `crew`
- Extract description (everything after `: `)
- Flag breaking changes: commits with `!` after type/scope or `BREAKING CHANGE:` in body

For commits that don't follow conventional format, group under "Other".

## Step 3: Group & Format

Generate markdown grouped by type:

```markdown
## [version] — YYYY-MM-DD

### Breaking Changes
- **scope**: description (hash)

### Features
- **scope**: description (hash)

### Bug Fixes
- **scope**: description (hash)

### Refactoring
- description (hash)

### Other
- description (hash)
```

Type mapping:
| Commit type | Section heading |
|-------------|-----------------|
| `feat` | Features |
| `fix` | Bug Fixes |
| `refactor` | Refactoring |
| `perf` | Performance |
| `docs` | Documentation |
| `test` | Tests |
| `chore` | Maintenance |
| `ci` | CI/CD |
| `style` | Style |

Omit empty sections. Use short hashes (7 chars).

## Step 4: Detect Version

If `pyproject.toml` or `package.json` exists, read the current version. Otherwise use `Unreleased` as the version placeholder.

## Step 5: Output

- **`--dry-run`** (default): Print the changelog entry to stdout without writing
- **Otherwise**: Prepend the entry to `CHANGELOG.md` (create if missing, preserve existing content below)

If the output path is specified via `--output`, write there instead.

## Step 6: Summary

```
## Changelog Generated

- Range: <previous-tag>..HEAD
- Commits: N total (X features, Y fixes, Z other)
- Breaking changes: N
- Output: CHANGELOG.md (or dry-run)
```
