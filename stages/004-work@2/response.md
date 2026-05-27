6470/6470 tests pass. Here's a summary of this pass:

## Summary

### Audit-flagged items fixed

Two items were explicitly called out by the previous audit:

1. **`fabro-telemetry/src/sanitize.rs:7`** — `"valid regex"` → `"hardcoded regex literal is always syntactically valid"` (matches the pattern used in `error.rs`)

2. **`fabro-auth/src/strategy.rs:68`** — Replaced bare `assert_eq!` with an explicit `if`/`panic!` block plus a comment explaining that all call sites enforce the CodexDevice+OpenAI invariant

### Additional weak messages fixed in this pass

The same scan surface found several more messages that failed the "explains why failure is impossible" test:

| File | Old message | Fix |
|------|-------------|-----|
| `generate.rs:213,232` | `"just pushed"` | `"steps is non-empty: element was pushed on the line above"` |
| `run_progress/styles.rs:13` | `"valid template"` | `"hardcoded progress template is always syntactically valid"` |
| `shared/utilities.rs:27` | `"valid template"` | same |
| `graphviz/parser/semantic.rs:98,118,132` | `"just ensured"` | `"node was just inserted by ensure_node, so get_mut cannot return None"` |
| `demo/mod.rs:232` | `"literal is non-zero"` | `"hardcoded literal 100 is non-zero"` |
| `slack/connection.rs:62` | `"ack serialization"` | explains the struct's derived `Serialize` has no custom logic that can fail |
| `stylesheet.rs:51` | `"node must exist"` | explains node_id was collected from the same map's keys |
| `event/convert.rs:654,713,745` | `"serializable agent/sdk error"` | explains the derives-based serialization has no custom failure path |
| `run_projection.rs:427` | `"event seq starts at 1"` | `"event sequence numbers are 1-based so seq is always non-zero at this call site"` |

### Remaining justified `panic!` calls (all verified)

- `tools.rs` — hardcoded tool name validation with `# Panics` doc
- `demo/mod.rs` — hardcoded demo SHA/status constants
- `strategy.rs` — programming invariant (CodexDevice+OpenAI pairing), all call sites verified

### Evidence

- `unwrap()` in production: **0**
- Weak `expect()` messages: **0**
- All `unreachable!`: documented with explicit structural invariants
- `cargo nextest run --workspace`: **6470 passed, 0 failed**