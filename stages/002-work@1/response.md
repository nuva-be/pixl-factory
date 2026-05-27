## Summary

This pass addressed the remaining production panic policy violations found after the previous remediation commit (`01892185f`).

### What was scanned

A Python-based analysis parsed all Rust source files, tracked `#[cfg(test)]` / `#[test]` block boundaries, and identified patterns in production code paths:
- `.unwrap()` → **0 remaining** in production code
- `panic!`, `todo!`, `unimplemented!`, `unreachable!` → reviewed all 20 occurrences; all were either justified (hardcoded constants, structural invariants with explanatory messages) or fixed
- `expect()` calls → reviewed all 400; identified 11 with weak/missing justifications

### Changes made

| File | Issue | Fix |
|------|-------|-----|
| `fabro-cli/src/commands/upgrade.rs` | `http_client().expect("failed to build HTTP client")` — HTTP client build can fail due to TLS/environment issues | Changed `select_backend_for_gh_command` and `select_backend` to return `Result<Backend>`, propagating the error with `?` to callers |
| `fabro-server/src/serve.rs` | `expect("failed to install Ctrl+C handler")` / `expect("failed to install SIGTERM handler")` — signal handler registration can fail in restricted environments | Replaced `expect` with logged warning + `std::future::pending()` fallback; if signal handlers can't be installed, the server continues without signal-based graceful shutdown |
| `fabro-telemetry/src/lib.rs` | `expect("failed to spawn telemetry thread")` — thread spawning can fail under resource pressure | Replaced with match + debug-level log; if the telemetry thread can't spawn, telemetry is silently disabled |
| `fabro-workflow/src/event/convert.rs` | `expect("run.created settings")` / `expect("run.created graph")` — described what fails, not why it's impossible | Updated messages to explain the invariant: "value was serialized from a typed struct in this session" |
| `fabro-server/src/auth/cli_flow.rs` (×2) | `expect("OS RNG")` — message too short, no justification | Updated messages to explain the security-critical invariant: "a failure indicates a broken system RNG that would compromise secret/auth code security" |
| `fabro-util/src/dev_token.rs` | `expect("OS RNG")` — same issue | Updated with same security-invariant justification |

### Remaining justified cases

- `fabro-llm/src/tools.rs` — `panic!` on invalid tool names: documented `# Panics`, tool names are hardcoded literals
- `fabro-server/src/demo/mod.rs` — `panic!` on invalid hardcoded SHA/status values: all demo data is source-tree constants
- All `unreachable!` calls — all have messages explaining the structural invariant that makes them unreachable

### Evidence
- `cargo build --workspace` → clean
- `cargo nextest run --workspace` → 6470 tests passed, 181 skipped, 0 failures