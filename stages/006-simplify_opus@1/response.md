## Summary

The three review agents flagged several issues; I implemented the high-value fixes and left low-value ones alone.

### Fixes applied

1. **Dedup'd dry-run coercion** — added `SandboxProvider::effective_for(mode)` in `fabro-types/src/sandbox_provider.rs`. Replaced the duplicated `if RunMode::DryRun && !is_local()` block in both `fabro-server/src/run_manifest.rs::effective_sandbox_provider` and `fabro-workflow/src/operations/start.rs` with a single method call. Also inlined the trivial one-line `resolve_sandbox_provider` wrapper in `run_manifest.rs`.

2. **Encapsulated the stringly-typed match** — added `ServerSandboxProvidersSettings::for_provider(SandboxProvider) -> &ServerSandboxProviderSettings` in `fabro-types/src/settings/server.rs`. `sandbox_provider_policy_error` now uses that instead of hand-matching variants.

3. **Extracted launch-time policy helper** — `fail_if_sandbox_provider_disabled` in `fabro-server/src/server.rs` replaces two identical 7-line blocks in `execute_run_in_process` and `execute_run_subprocess` (including the duplicated `tracing::error!` message).

4. **Looped installer writes** — `write_sandbox_provider_policy` in `fabro-install/src/lib.rs` now iterates over `[SandboxProvider::Local, Docker, Daytona]` (via `Display`) instead of three hardcoded string-literal calls. Removed the now-redundant `write_sandbox_provider_enabled` helper.

### Findings deliberately skipped

- **Collapsing the 3-struct hierarchy** to a flat `{ local: bool, docker: bool, daytona: bool }` — keeping the per-provider struct preserves room for future per-provider fields and matches the TOML shape.
- **Returning `Result<(), String>` from `sandbox_provider_policy_error`** — the four call sites use the error in different ways (response body, check report, run-fail message), so `Option<String>` is the right shape.
- **Backtick-quoting the config path in the error message** — the spec mandates the exact string, and the new tests assert it verbatim.
- **Hoisting `state.server_settings()` in `create_run`** — `RwLock::read + Arc::clone` is negligible.
- **Pulling `assert_sandbox_provider_policy_enabled` into a shared test-support module** — only used in two tests of one crate.

### Verification

- `cargo check --workspace` clean
- `cargo nextest run -p fabro-config` 220/220 pass (incl. new policy tests)
- `cargo nextest run -p fabro-types -p fabro-install` 326+install pass
- `cargo nextest run -p fabro-server` 631/634 pass — the 3 failures (`get_graph_returns_svg` + 2 related) are pre-existing and reproduced on the parent commit before any cleanup edits
- `cargo nextest run -p fabro-server --test it --features test-support` 143/143 pass
- `cargo nextest run -p fabro-workflow` 1162/1162 pass
- `cargo nextest run -p fabro-api` 153/153 pass (incl. round-trip)
- `cargo +nightly-2026-04-14 fmt --check --all` clean
- `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings` clean

Net change: **+76/−48 lines** removed across 6 files (further reducing duplication on top of the original feature commit).