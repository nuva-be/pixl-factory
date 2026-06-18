---
name: grill
description: "Stress-test a plan, proposal, or assumption BEFORE committing - one question at a time, cross-checking answers against actual code and inventing missed edge cases. Use to \"grill me on this\", \"stress-test the plan\", or \"poke holes in this\"."
version: 1.0.0
allowed-tools: Read, Grep, Glob, Bash
argument-hint: "[topic | --plan <file>]"
---

## When to use

Ported from Matt Pocock's grill-with-docs methodology. Trigger phrases: "grill me on this", "stress-test the plan", "challenge my assumptions", "before we commit to X", "interview me about", "what edge cases are we missing", "poke holes in this".

# /grill — one-question interrogation

A planning-hardening skill. The goal is to expose unstated assumptions, missing edge cases, and reasoning that won't survive contact with the codebase — BEFORE a single line of implementation is written.

## Core discipline — ONE question at a time

Never bundle questions. Ask ONE focused question, wait for the answer, cross-check it against actual code, THEN move to the next. Bundling lets the author skip the hardest one.

Bad: "What happens when input is empty, null, or concurrent? Also how do you handle multi-tenant boundaries?"
Good: "What happens when `input.items` is an empty array — do we 200 with `[]` or 400?" → wait → cross-check → next.

## Cross-check before accepting (mandatory)

Every answer the author gives must be verified against the codebase before moving on. The author will be confident about things the code contradicts.

1. Read the answer.
2. Use Read/Grep/Glob to find the actual code path the answer describes.
3. If the code matches → accept and move to the next question.
4. If the code contradicts → quote the file and lines, ask the author to reconcile.
5. If the code is silent → flag "no code path exists yet" and note it as a real risk, not a hypothetical.

This is the difference between grilling and chatting. Without the cross-check it's theatre.

## Invent edge cases the author hasn't considered

For each topic, generate at least one question from each category — only ask the ones the author hasn't already addressed:

- **Empty / null / zero** — empty list, null user, 0-amount payment, missing optional field
- **Concurrent access** — two writers, race between read-modify-write, double-submit
- **Malicious input** — oversized payload, SQL/template injection, unicode tricks, path traversal
- **Network failure** — timeout mid-call, partial write, retry storm, idempotency
- **Multi-tenant boundary** — user A's request returns user B's data, cross-tenant search leak
- **Time / ordering** — clock skew, DST, expired token used at exact expiry second, out-of-order events
- **Scale** — 10x the expected volume, the one user with 10k records, the cold-start latency
- **Reversal** — what does undo / rollback / refund / delete look like? Is it possible at all?

## The three-conditions ADR rule

Do NOT suggest creating an Architecture Decision Record for every grilling session. An ADR is only worth the overhead when ALL THREE hold:

1. **Costly to reverse** — undoing this decision later requires data migration, breaking API change, or rewriting >1 module.
2. **Reasoning is non-obvious** — six months from now, a maintainer reading the code alone would not understand why this choice was made.
3. **Real alternatives were evaluated** — at least one credible alternative was on the table and rejected for documented reasons. "We picked X because X was the only option we considered" does not qualify.

If any of the three is missing, propose a code comment or a short note in the relevant README — not an ADR. Most grilling sessions end without an ADR, and that is correct.

## Anchor on CONTEXT.md if present

If `CONTEXT.md` exists at the repo root, Read it first and use its vocabulary for the entire session. The point of CONTEXT.md is that "checkout", "order", "tenant", "user" have agreed-upon definitions in this codebase — drifting away from that vocabulary during grilling produces answers that don't map back to the code. Quote terms from CONTEXT.md verbatim when asking questions.

If `CONTEXT.md` does not exist, note it ONCE at session start ("no domain glossary present — vocabulary mismatches are likelier") and continue.

## Session flow

1. **Frame** — one sentence on what is being grilled: a plan doc, a proposed PR, a verbal idea. If `--plan <file>` was passed, Read it.
2. **Identify the riskiest claim first** — not the most interesting, the one whose failure would hurt most.
3. **Ask ONE question** — anchored in code or CONTEXT.md vocabulary.
4. **Cross-check the answer** — Read/Grep until you have evidence.
5. **Record the outcome** — `[CONFIRMED]`, `[CONTRADICTED by <file>:<line>]`, or `[GAP — no code path]`.
6. **Pick the next riskiest claim** — repeat. Stop when all surviving claims are `[CONFIRMED]` or `[GAP]` with the author acknowledging the gap.
7. **Summarize** — list `[GAP]` items as real work, list `[CONTRADICTED]` items as plan-must-change-before-coding items. Apply the three-conditions ADR rule.

## Output shape

```text
GRILL — <topic>

Q1: <one question, anchored>
A: <author answer>
CHECK: Read src/x.ts:42–58 → [CONFIRMED | CONTRADICTED | GAP]

Q2: ...

SURVIVING ASSUMPTIONS
  [CONFIRMED] ...
  [GAP] ...           ← real work
  [CONTRADICTED] ...  ← plan must change

ADR RECOMMENDATION
  <none | one-liner if all three conditions met>
```

## What this skill is NOT

- Not a code review — see `/code-review` for diffs.
- Not a security audit — see `/security-scan` for OWASP coverage.
- Not a planning skill — see `/task-plan` to break work down. Grill the plan, then plan the work.
- Not a debate. The author is not the adversary; the unstated assumption is.

## Related

- `/task-plan` — break a feature into ordered tasks once grilling is done
- `/spec-review` — compare an implementation against its PRD
- `/cto-review` — architectural simplification pass after the plan survives grilling
- `references/books/` — Feathers (Legacy Code), Hunt & Thomas (Pragmatic Programmer) for further reading on stress-testing assumptions
