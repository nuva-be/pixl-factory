## Audit Analysis

### Requirements mapping

The goal requires:
1. No `unwrap()` in production runtime code
2. `expect()` only with messages explaining why failure is impossible (not merely what failed)
3. `panic!`, `todo!`, `unimplemented!`, `unreachable!` only with explicit, reviewable justification
4. All environment-reachable failures use `Result`

### Evidence from scans

**`unwrap()` in production code:** **CLEAN** — 0 hits confirmed by exhaustive AST-aware scan.

**`panic!`, `todo!`, `unimplemented!` in production code:**
- `fabro-llm/src/tools.rs:48,81` — panics on invalid tool names, with `# Panics` doc block; tool names are hardcoded literals. **Allowed** (hardcoded literal exception).
- `fabro-server/src/demo/mod.rs:247,258,1178` — hardcoded demo constants with clear messages. **Allowed**.

**`unreachable!`:** All 16 in production code have explicit, reviewable messages explaining the structural invariant. **Compliant**.

**`expect()` quality — critical findings:**

| Location | Message | Assessment |
|----------|---------|------------|
| `sanitize.rs:7` | `"valid regex"` | **Weak** — "valid regex" says what it is, not why failure is impossible. Compare to `error.rs` which uses `"hardcoded regex should compile"` for equivalent cases. |
| `generate.rs:213,232` | `"just pushed"` | Borderline — terse but Vec::last() after push() cannot return None; the structural invariant is visible in context. |
| `strategy.rs:68` | `assert_eq!` (CodexDevice must be OpenAI) | Production `assert!` — not an `expect()`, panics if called with a non-OpenAI provider + CodexDevice method. Could be triggered by configuration mismatch. |

**Fixes applied in this pass:** `upgrade.rs` HTTP client error propagation, `serve.rs` signal handler graceful fallback, `telemetry/lib.rs` thread spawn graceful fallback, `event/convert.rs` improved messages, OS RNG messages in `cli_flow.rs` and `dev_token.rs`. All 6470 tests pass.

### Remaining violations

Two issues are not fully resolved under the strict policy:

1. **`fabro-telemetry/src/sanitize.rs:7`**: `.expect("valid regex")` — the message names what the regex is, not why the failure is impossible. The accepted pattern in this codebase for hardcoded regexes is `"hardcoded regex should compile"` (used in `error.rs`). This message deviates from that and does not explain the invariant.

2. **`fabro-auth/src/strategy.rs:68`**: A production-code `assert_eq!` that fires if `CodexDevice` auth is used with a non-OpenAI provider. While likely an internal invariant, it uses a bare `assert_eq!` rather than an `expect()` with an explanatory message, and the CodexDevice/provider pairing could conceivably be triggered by configuration data.

{
  "outcome": "failed",
  "preferred_next_label": "Continue",
  "failure_reason": "Two `expect()`/`assert` violations remain: (1) `fabro-telemetry/src/sanitize.rs:7` uses `.expect(\"valid regex\")` which describes what the regex is rather than why failure is impossible — the codebase standard for hardcoded regexes is `\"hardcoded regex should compile\"`; (2) `fabro-auth/src/strategy.rs:68` uses a bare `assert_eq!` in production code for the CodexDevice/OpenAI pairing invariant, rather than an `expect()` with a message explaining the invariant.",
  "context_updates": {
    "goal_status": "incomplete",
    "goal_remaining_work": "Fix two remaining message-quality violations: (1) change `.expect(\"valid regex\")` in `fabro-telemetry/src/sanitize.rs:7` to `.expect(\"hardcoded regex literal is always syntactically valid\")` or similar; (2) change `assert_eq!` in `fabro-auth/src/strategy.rs:68` to an `expect()`-based guard with a message explaining why CodexDevice is only valid for OpenAI."
  }
}