---
name: investigate
version: 0.1.0
description: "Root-cause debugging with enforced investigation before fixing - a 5-phase method (deterministic signal, reproduce, diagnose, hypothesize, fix). Use when a failing test, broken feature, or mysterious error needs reproduce-then-diagnose work."
allowed-tools: Read, Bash, Glob, Grep, Edit, Write, Agent
argument-hint: "<symptom or error message>"
---

## When to use

Trigger phrases: "this test is failing", "investigate this bug", "find the root cause", "why is this broken", "debug this error".

## Overview

Structured debugging that enforces investigation before fixing. Inspired by the "3-strike rule" — you must understand the root cause before writing any fix. Every fix requires a regression test.

## Phase 1 — Build a deterministic signal FIRST

**Before reproducing the bug, build the pass/fail oracle that decides whether the bug is present.** Without a deterministic signal you cannot bisect, cannot trust a fix, and cannot prevent regression — it is the most-skipped debugging discipline and the most common cause of "the fix didn't work" loops.

A signal exits `0` when behavior is correct, non-zero when the bug is present. Pick the cheapest form: a single `pytest <file>::<test>` command, a `./repro.sh` one-liner, a `curl -fsS ... | jq -e <assert>`, or a stress loop for intermittents (`pytest --count=20`, exit non-zero on any failure).

**Two-sided verification is mandatory.** Both sides must be proven before you continue:

1. Run on the current broken code → must FAIL (non-zero). If it passes, the signal does not capture the bug — refine it.
2. Run on a known-good ref (`git stash`, `git checkout <last-good-sha>`, or revert the suspected commit) → must PASS (exit 0). If it also fails, the signal is over-broad and would not catch a regression — narrow it.

This signal is what `git bisect run` consumes, what the Phase 5 regression test asserts, and what the fix is measured against. Do NOT skip this phase even when the bug "obviously reproduces."

Output:
```
SIGNAL:      <one-line command or script path>
FAILS ON:    <current SHA / "HEAD"> — exit <N>, message <quote>
PASSES ON:   <known-good SHA / "HEAD~5"> — exit 0
DETERMINISM: <ran 5x — 5/5 same result | intermittent — see notes>
```

**How this differs from other skills**:
- `/runbook` — live incident response for production systems. This skill is for code-level debugging.
- `/self-review-fix-loop` — iterative review-and-fix loop for open changes. This skill starts from a symptom and traces to root cause.
- `/test-runner` — runs tests. This skill debugs WHY tests (or features) fail.

## Capability matrix

```
┌─────────────────────────────────────────────────────────────────┐
│                          /investigate                           │
├─────────────────────────────────────────────────────────────────┤
│  STANDALONE (always works)                                      │
│  ✓ Reproduce symptom via tests, Bash, Read                      │
│  ✓ Trace call graph via Grep/Glob                               │
│  ✓ Hypothesize → bisect → narrow to root cause                  │
│  ✓ Write regression test before applying fix                    │
├─────────────────────────────────────────────────────────────────┤
│  SUPERCHARGED (when these are available)                        │
│  + pixl knowledge search: AST-aware code traversal              │
│  + LSP (typescript/pyright/swift-lsp): true find-references     │
│  + git bisect: pinpoint introducing commit automatically        │
│  + Sentry MCP: pull stack trace + breadcrumbs from prod         │
│  + PostHog MCP: cross-check user session replay                 │
│  + GitHub MCP: link to related PR, recent change history        │
└─────────────────────────────────────────────────────────────────┘
```

Without supercharged deps the skill still enforces the 4-phase methodology using Grep/Read/Bash — connectors only accelerate hypothesis formation and add production evidence.

## Decision rules from the SWE classics

This skill applies decision rules distilled from the following book (vendored under `references/books/`):

- **Working Effectively with Legacy Code** (Feathers) — [mini](../../references/books/working-effectively-with-legacy-code/working-effectively-with-legacy-code.mini.md): Characterization Tests (pin existing behavior before changing), Sprout Method (add the fix in a new method to avoid touching legacy seams), identifying Seams. Use in Phase 4 to write the regression test BEFORE applying the fix.

Cite "Characterization Test — Feathers" when the regression test exists to pin current behavior, and "Sprout Method — Feathers" when the fix lives in a new method rather than mutating the legacy path.

## Example

**Input**: `/investigate "test_checkout::test_apply_discount_code fails intermittently with KeyError 'subtotal'"`

**What happens**:

1. **Phase 1 (signal)**: Builds `pytest tests/test_checkout.py::test_apply_discount_code --count=20` as the oracle — fails on HEAD (4/20), passes on `HEAD~3` (0/20) → signal proven both sides
2. **Phase 2 (reproduce)**: Re-runs the signal — confirms intermittent (3/10 fails)
3. **Phase 3 (diagnose)**: Reads stack trace, traces data flow from `apply_discount_code()` to the KeyError; reads recent commits touching `checkout.py`
4. **Phase 4 (hypothesize)**: Forms hypothesis — cart object is mutated by a parallel test; cart fixture is module-scoped, not function-scoped
5. **Phase 5 (fix)**: Writes regression test pinned at the fixture seam, applies the fix (fixture scope change), verifies the Phase 1 signal now passes, writes a commit message recording the winning hypothesis and what was ruled out

**Output**:

```text
INVESTIGATE — test_apply_discount_code intermittent KeyError

PHASE 1 — SIGNAL
  Signal:    pytest tests/test_checkout.py::test_apply_discount_code --count=20 -q
  Fails on:  HEAD                  → 4/20 runs fail (exit 1)
  Passes on: HEAD~3 (before a3f9d2) → 20/20 pass (exit 0)
  Determinism: ran 3x — consistent fail rate

PHASE 2 — REPRODUCE
  Symptom:     KeyError: 'subtotal'
  Reproduction: pytest tests/test_checkout.py::test_apply_discount_code --count=10
                → 3 of 10 runs fail (30%)
  Expected:    cart.totals['subtotal'] = 100.00
  Actual:      cart.totals = {} (subtotal key deleted by another test)

PHASE 3 — DIAGNOSE
  Stack: tests/test_checkout.py:42 → src/checkout.py:88 → cart.totals['subtotal']
  Recent change: commit a3f9d2 (3 days ago) — added test_cart_clear_totals
                 which calls cart.totals.clear()
  Root cause:    `cart` fixture is `scope="module"` in conftest.py:14
                 → test_cart_clear_totals empties it; subsequent test sees {}

PHASE 4 — HYPOTHESIS
  Change fixture to `scope="function"` so each test gets a fresh cart.

PHASE 5 — FIX
  + tests/test_checkout.py:32 — new regression test (forces order: clear then apply)
  ~ conftest.py:14 — @pytest.fixture(scope="function")

Verification: pytest tests/test_checkout.py --count=20 → 20/20 pass
```

## Phase 2: Reproduce (Read-Only)

**Goal**: Confirm the bug exists and define the exact failure condition.

1. **Capture the symptom**: What exactly is failing? Error message, unexpected behavior, test failure?
2. **Reproduce reliably**: Find the minimal steps to trigger the bug
   - Run the failing test: `pytest <file>::<test> -v --tb=long` or `vitest run <file> -t "<test>"`
   - Or reproduce manually via the described steps
3. **Define the contract**: What SHOULD happen vs what DOES happen?
4. **Scope the blast radius**: Is this isolated or does it affect other areas?

Output:
```
SYMPTOM: <exact error or unexpected behavior>
REPRODUCTION: <minimal steps to trigger>
EXPECTED: <what should happen>
ACTUAL: <what does happen>
SCOPE: <isolated | cross-cutting | unknown>
```

**Do NOT write any fixes yet.** Phase 2 is strictly read-only.

## Phase 3: Diagnose (Read-Only)

**Goal**: Narrow to a single root cause. Follow the evidence, not assumptions.

### Diagnostic Ladder

Work through in order — stop when you find the root cause:

1. **Read the error**: Parse the full stack trace. What line, what function, what input?
2. **Trace the data flow**: Follow the input from entry point to failure point
   - Use Grep to find callers: `grep -r "functionName" --include="*.ts"`
   - Read each file in the call chain
3. **Check recent changes**: `git log --oneline -20 -- <affected-files>` and `git diff HEAD~5 -- <affected-files>`
4. **Compare working vs broken**: If it worked before, what changed? `git bisect` if needed
5. **Check dependencies**: Version changes, config changes, environment differences
6. **Inspect state**: Add temporary logging or use debugger to inspect runtime values

### Diagnostic Rules

- **No guessing**: Every hypothesis must be backed by evidence from code or logs
- **One variable at a time**: Don't change multiple things while debugging
- **Read before assuming**: Always read the actual code — don't assume you know what it does
- **Check the obvious first**: Typos, wrong variable names, missing imports, stale caches

Output:
```
ROOT CAUSE: <specific technical explanation>
EVIDENCE: <what code/logs/behavior proves this>
AFFECTED CODE: <file:line references>
CONFIDENCE: <high | medium | low>
```

**Do NOT write any fixes yet.** Phase 3 is strictly read-only.

## Phase 4: Hypothesize

**Goal**: Design the fix before implementing it.

1. **Propose the fix**: What specific code change will resolve the root cause?
2. **Assess risk**: Could this fix break anything else?
3. **Identify the test**: What regression test will prevent this bug from recurring?
4. **Consider alternatives**: Is there a simpler fix? A more robust fix?

If confidence from Phase 3 is "low":
- Spawn an Explore agent to search for similar patterns in the codebase
- Check if this is a known issue pattern (search error messages, check issues)
- If still uncertain, present findings to the user before proceeding

Output:
```
FIX: <specific change description>
FILES: <files to modify>
RISK: <what could break>
TEST: <regression test to write>
ALTERNATIVE: <simpler or more robust option, if any>
```

## Phase 5: Fix + Verify

**Goal**: Implement the fix with a regression test.

### 5a: Write the Regression Test First (TDD Red)

Write a test that:
- Reproduces the exact bug (fails before the fix)
- Verifies the correct behavior (passes after the fix)
- Is specific enough to catch regressions

Run the test — confirm it FAILS for the right reason.

**Pin the test at the right architectural seam** — where the bug actually lives, not where the test is easiest to write. Pure fn → unit; query/repo → integration with real DB; request/serialization/middleware → route-level; workflow/job/state machine → orchestrator, not leaf. A test pinned at the wrong seam passes when the bug returns through a different path. Cite "Characterization Test — Feathers" in the test comment when pinning legacy behavior.

### 5b: Apply the Fix

Make the minimum change needed to fix the root cause. Do not:
- Refactor surrounding code
- Fix unrelated issues
- Add features
- Change things "while you're in there"

### 5c: Verify

1. Run the new regression test — confirm it PASSES
2. Run the Phase 1 deterministic signal — confirm it now exits 0
3. Run related tests — confirm nothing broke: `pytest <directory> --tb=short -q` or `vitest run <directory>`
4. Re-check the original symptom — confirm it's resolved
5. If the fix doesn't work, return to Phase 3 — do NOT layer fixes on top of fixes

### 5d: Commit message records which hypothesis won

The commit must document (a) the Phase 4 hypothesis that proved correct, and (b) hypotheses that were ruled out and why — so the next debugger does not re-walk the same dead ends.

```
fix(<scope>): <one-line summary>

Symptom: <Phase 2>
Root cause: <Phase 3 finding, file:line>
Hypothesis confirmed: <Phase 4 option that worked>
Ruled out: <other hypotheses + reason rejected>
Regression test: <path + seam pinned>
Signal: <Phase 1 command that now passes>
```

## Anti-Patterns

- **Shotgun debugging**: Changing random things hoping something works. Return to Phase 2.
- **Fix layering**: Adding a second fix on top of a failed first fix. Revert and start over.
- **Symptom fixing**: Suppressing the error instead of fixing the cause (try/catch around the bug, `|| true`, ignoring the return value).
- **Over-fixing**: Refactoring the entire module when one line was wrong.
- **Skipping reproduction**: "I think I know what's wrong" — always reproduce first.

## Gotchas

- Intermittent bugs need multiple reproduction attempts — don't declare "can't reproduce" after one try
- Race conditions require stress testing: run the test 10x in a loop
- Environment-dependent bugs: check Node/Python version, OS, env vars, .env files
- If `git bisect` points to a merge commit, dig into the individual commits within it
- Memory/state bugs may not manifest in test isolation — check for global state pollution
