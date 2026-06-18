---
name: runbook
version: 0.1.0
description: "Guide systematic production incident response: symptom, diagnosis, remediation, postmortem. Use to \"production is down\", \"payments are degraded\", \"investigate this incident\", \"write a runbook\", or \"do a postmortem\"."
allowed-tools: Read, Bash, Grep, Glob
argument-hint: "<service> [--postmortem]"
---

## Overview

Two modes:
- **Investigate**: Live incident response — follow the triage ladder to isolate root cause fast
- **Document**: Generate a runbook for a service so the next incident takes minutes, not hours

## Capability matrix

```
┌─────────────────────────────────────────────────────────────────┐
│                            /runbook                             │
├─────────────────────────────────────────────────────────────────┤
│  STANDALONE (always works)                                      │
│  ✓ Triage ladder: blast radius, recent deploy, suspect commits  │
│  ✓ git log/blame correlation with incident timeline             │
│  ✓ Local log scan via Grep across relevant services             │
│  ✓ Generate static runbook.md for a service                     │
│  ✓ Author postmortem from gathered evidence                     │
├─────────────────────────────────────────────────────────────────┤
│  SUPERCHARGED (when these are available)                        │
│  + Sentry MCP: live error rate, breadcrumbs, Seer AI hypotheses │
│  + PostHog MCP: user impact, funnel disruption, replay sessions │
│  + Cloudflare/Supabase MCP: read live logs and infra status     │
│  + GitHub MCP: pull deploys, recent PRs, on-call rotation       │
│  + Linear MCP: open incident ticket + link postmortem           │
└─────────────────────────────────────────────────────────────────┘
```

Without supercharged deps the skill still walks the triage ladder using git + local logs and writes a runbook — connectors give live error signal, user-impact data, and one-click ticketing.

## Decision rules from the SWE classics

This skill applies decision rules distilled from the following book (vendored under `references/books/`):

- **Release It!** (Nygard) — [mini](../../references/books/release-it/release-it.mini.md): Stability Patterns (Circuit Breaker, Bulkhead, Timeout, Steady State, Fail Fast) and antipatterns (Cascading Failures, Blocked Threads, Integration Points). Use when diagnosing the failure mode and recommending durable fixes (not just rollbacks).

Cite the named pattern/antipattern (e.g. "Cascading Failure via shared connection pool — Nygard") in the diagnosis so the postmortem has shared vocabulary.

## Example

**Input**: `/runbook payments` (live incident — payment API returning 500s)

**What happens**:

1. **Phase 1 (triage, ~5 min)**: Determines blast radius — "all card payments failing, started 12 minutes ago, coincides with deploy a8c4f2"
2. **Phase 2 (diagnose, 5-20 min)**: Greps app logs for stack traces, finds `ConnectionPoolExhausted` from Stripe webhook handler; checks recent deploy diff
3. **Phase 3 (remediate)**: Identifies hotfix (revert) and proper fix (increase pool + add timeout); recommends revert first
4. **Phase 4 (postmortem)**: When called with `--postmortem`, generates structured doc from the timeline + evidence captured during the incident

**Output** (live incident triage):

```text
INCIDENT — payments service degraded

PHASE 1 — TRIAGE (3 min)
  Blast radius:  All card payments (Stripe-backed) | ~140 affected users/min
  Started:       12:34 UTC (12 min ago) | coincides with deploy a8c4f2 at 12:33
  Impact:        Revenue blocked. ESCALATE (payment processing affected).

PHASE 2 — DIAGNOSE (8 min)
  Logs:          47 × ConnectionPoolExhausted in last 10 min
  Stack:         src/payments/stripe-webhook.ts:88 → db.transaction(...)
  Diff a8c4f2:   added retry loop without releasing connection on failure
                 (each retry holds a connection until exhaustion)

PHASE 3 — REMEDIATE
  Immediate:     `gh pr merge --revert <pr-number>` → redeploy
                 Validation: webhook 200s resume within 2 min
  Proper fix:    src/payments/stripe-webhook.ts — wrap retry in try/finally
                 to release connection; add 5s timeout per attempt

NEXT: `/runbook payments --postmortem` to generate the postmortem doc.
```

## Mode Detection

```
/runbook payments          → investigate live incident in payments service
/runbook payments --postmortem  → document lessons after the incident
/runbook api               → triage an API degradation
```

## Investigate Mode

### Phase 1: Triage (first 5 minutes)

**Goal**: Determine blast radius and whether to escalate immediately.

1. What is failing? (service, endpoint, feature, region, all users vs subset)
2. When did it start? (`git log --since`, deploy timestamps, monitoring alerts)
3. What changed? (recent deploys, config changes, dependency updates)
4. What is the user impact? (errors returned, data corruption, performance degradation)

**Escalate immediately if**: data loss possible, payment processing affected, auth broken for all users.

### Phase 2: Diagnose (5–20 minutes)

**Goal**: Narrow to a single root cause hypothesis.

Check in order:
1. **Application logs** — look for error spikes, stack traces, new error codes
2. **Infrastructure** — CPU, memory, disk, connection pools, queue depths
3. **Dependencies** — database latency, external API errors, cache hit rates
4. **Recent changes** — diff between last good deploy and current

Useful commands:
```bash
# Application logs (last 100 errors)
grep -i "error\|exception\|fatal" app.log | tail -100

# Process health
ps aux | grep <service>
# or: systemctl status <service>

# Port/connection check
lsof -i :<port>
netstat -an | grep <port>

# Disk space
df -h
du -sh /* | sort -rh | head -10
```

### Phase 3: Remediate

**Goal**: Restore service — accept temporary degradation over outage.

Remediation ladder (try in order):
1. **Rollback** — fastest if a recent deploy caused it: `git revert HEAD` or redeploy previous image
2. **Restart** — clears memory leaks, zombie processes: `systemctl restart <service>` or `pm2 restart all`
3. **Scale** — if load-caused: increase replicas/instances
4. **Circuit break** — disable the failing feature/dependency to restore partial service
5. **Hotfix** — only if rollback is not possible and root cause is confirmed

Document what you tried and what the result was.

### Phase 4: Verify

1. Confirm the symptom is gone (check the same metric that showed the failure)
2. Verify no secondary failures introduced by the fix
3. Monitor for 15 minutes before declaring resolved

## Document Mode (--postmortem)

Generate a runbook from service analysis:

1. Scan the service for: startup sequence, health check endpoints, key dependencies, config requirements
2. Produce a runbook with: service overview, common failure modes, remediation steps, monitoring links

Output template:
```markdown
# Runbook: <service-name>

## Service Overview
- **Purpose**: ...
- **Owner**: ...
- **Health check**: `curl <url>/health`
- **Logs**: `<log location>`
- **Deploy**: `<deploy command>`

## Common Failures

### <Failure 1>
**Symptoms**: ...
**Cause**: ...
**Fix**: `<command>`

### <Failure 2>
...

## Escalation
- P1 (data loss, payments): page on-call immediately
- P2 (degraded, partial): Slack #incidents within 15 min
- P3 (cosmetic, single user): ticket during business hours
```

## Gotchas
- Don't optimize during an incident — restore first, improve later
- Rollback is almost always faster than hotfix — default to rollback
- Document every remediation attempt in real time — memory degrades under stress
- "It was working before" is not a diagnosis — find the specific change
- Check infrastructure before application — infra failures masquerade as app bugs
