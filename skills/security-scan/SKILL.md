---
name: security-scan
version: 0.1.0
description: "Automated security scanning: secrets detection, OWASP Top 10 checks, dependency CVEs, and RBAC audit. Use to \"security audit\", \"scan for vulnerabilities\", \"check for leaked secrets\", \"OWASP review\", or \"audit our RBAC\" before a release."
allowed-tools: Read, Bash, Glob, Grep, Agent
argument-hint: "<scope: secrets|owasp|deps|rbac|full> [path]"
---

# Security Scan

Automated security scanning across multiple dimensions.

## Setup

Read `config.json` in this skill directory (if it exists):
- `scan_scope` — default scope when none given; default "full"
- `severity_threshold` — minimum severity to report; default "MEDIUM"
- `skip_modules` — directories to exclude; default ["node_modules", ".next", "dist", "__pycache__"]

If invoked without arguments, use `scan_scope` as the default. Override with the argument if provided.

## Capability matrix

```
┌─────────────────────────────────────────────────────────────────┐
│                       /security-scan                            │
├─────────────────────────────────────────────────────────────────┤
│  STANDALONE (always works)                                      │
│  ✓ Secret detection via regex (Stripe, AWS, GH, Slack, PEM)     │
│  ✓ OWASP Top 10 pattern checks (SQLi, XSS, SSRF, path traversal)│
│  ✓ RBAC route inspection (missing guards, tenant isolation)     │
│  ✓ Severity classification (CRITICAL / HIGH / MEDIUM / LOW)     │
│  ✓ Markdown report with line refs                               │
├─────────────────────────────────────────────────────────────────┤
│  SUPERCHARGED (when these are available)                        │
│  + npm audit / pip audit: dependency CVE scoring                │
│  + trailofbits/static-analysis: Semgrep + CodeQL deep checks    │
│  + trailofbits/supply-chain-risk-auditor: maintainer + CVE risk │
│  + Sentry MCP: cross-reference exploits seen in prod            │
│  + gh CLI: post findings as PR review comments                  │
└─────────────────────────────────────────────────────────────────┘
```

Without npm/pip audit, `deps` scope falls back to a `package.json` / `requirements.txt` inspection only. Without trailofbits plugins, no deep static analysis — only regex/pattern matching. Without Sentry, no prod exploit correlation.

## Example

**Input**: `/security-scan full`

**What happens**:

1. Runs `secrets` scope — greps for `sk_live_`, AWS keys, `BEGIN.*PRIVATE KEY`, hardcoded passwords
2. Runs `owasp` scope — pattern matches for SQLi, XSS, SSRF, missing auth middleware
3. Runs `deps` scope — `npm audit --json` + `pip audit --format json`
4. Runs `rbac` scope — inspects route definitions for missing permission guards
5. Classifies findings by severity, writes markdown report with file:line refs

**Output**:

```text
Security Scan — full | 47 files scanned

CRITICAL (2)
  src/lib/stripe.ts:8 — Hardcoded sk_live_xxx... key in source
  src/api/admin.ts:42 — DELETE /admin/users/:id has no auth guard

HIGH (3)
  src/db/query.ts:67 — SQL string concatenation: `SELECT * FROM users WHERE email='${email}'`
  src/api/proxy.ts:23 — User-controlled URL passed to fetch() (SSRF)
  package.json — lodash@4.17.15 has GHSA-jf85-cpcp-j695 (prototype pollution)

MEDIUM (5)
  src/components/Comment.tsx:34 — dangerouslySetInnerHTML on user input
  ...

Total: 2 critical, 3 high, 5 medium, 8 low. Block release until critical/high resolved.
```

## Quick Start

1. **Parse scope** — `secrets`, `owasp`, `deps`, `rbac`, or `full`. Optional path arg narrows scanning.
2. **Run scope-specific scans** — each scope has its own grep patterns and tool calls:
   - secrets → grep for API keys, private keys, hardcoded passwords, `.env` in git
   - owasp → patterns for SQLi, XSS, SSRF, missing auth, CORS, path traversal, JWT secrets
   - deps → `npm audit --json` / `pip audit` / `govulncheck`
   - rbac → find route definitions, verify auth middleware present
3. **Classify each finding** — CRITICAL / HIGH / MEDIUM / LOW. Validate before reporting (regex secrets have false positives).
4. **Write report** — markdown summary table + per-category findings with file:line + remediation.
5. **Log to history** — append a JSONL record under `${CLAUDE_PLUGIN_DATA}/security-scan/scan-history.jsonl` for trend tracking.

For grep commands, OWASP pattern table, RBAC checklist, gotchas, helper scripts, and the full report template, read [`reference.md`](./reference.md).

## Verify

After applying any remediation suggested by the scan, re-run the scan and confirm the count dropped. Surface failures to the user before declaring done.

1. **Baseline saved** — confirm the initial scan report was written and counts recorded:
   ```bash
   ls .claude/reports/security-scan-*.md | tail -n 1
   jq -s 'last' "${CLAUDE_PLUGIN_DATA:-.}/security-scan/scan-history.jsonl"
   ```
   Expected: report file exists; history JSONL has the latest run with `critical`, `high`, `medium`, `low` counts.

2. **Re-scan after fixes** — run the same scope again:
   ```bash
   /security-scan <same-scope> <same-path>
   ```
   Capture the new report.

3. **Counts decreased on every fixed category** — compare before vs after:
   ```bash
   jq -s '.[-2:] | {before: .[0], after: .[1]}' "${CLAUDE_PLUGIN_DATA:-.}/security-scan/scan-history.jsonl"
   ```
   Expected:
   - CRITICAL count == 0 (no remaining critical findings).
   - HIGH count strictly lower than the baseline (unless explicitly waived with documented rationale).
   - No NEW finding appears at a higher severity than the baseline (no regressions introduced by fixes).

4. **Spot-check each remediated finding** — for every CRITICAL/HIGH item the scan flagged as fixed:
   ```bash
   grep -nE '<original vulnerable pattern>' <file>
   ```
   Expected: no match. The original pattern must be gone (not merely commented out).

5. **Dependency CVEs cleared** — when `deps` was in scope and packages were bumped:
   ```bash
   npm audit --audit-level=high --json | jq '.metadata.vulnerabilities'
   uv pip-audit --strict || pip-audit --strict
   ```
   Expected: 0 high-or-critical advisories.

6. **Auth + RBAC probes** — when `rbac` was in scope and guards were added, re-probe the previously-vulnerable routes:
   ```bash
   curl -o /dev/null -w "%{http_code}\n" http://localhost:3000/<route>
   ```
   Expected: HTTP 401/403 for anonymous; HTTP 200 only for authorized roles.

If any step fails, surface the failure and do NOT declare the scan resolved. Unfixed CRITICAL/HIGH findings block release.

## See also

- [`reference.md`](./reference.md) — per-scope grep patterns, gotchas, data logging, report template
- `references/standards/security-audit.md` — OWASP patterns + severity classification (read before starting)
- `scripts/scan-secrets.sh`, `scripts/scan-owasp.sh` — helper scripts for quick triage
