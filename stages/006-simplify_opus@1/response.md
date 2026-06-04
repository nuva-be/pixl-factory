All 6503 workspace tests pass. Summary of the cleanup:

## Code Review Findings and Fixes

### Findings

**Code reuse (high-priority):**
- 8 frontend test files duplicated the same inline `TEST_PRINCIPAL`/`testPrincipal()` definition: `app/data/runs.test.ts`, `app/lib/run-actions.test.ts`, `app/routes/automations-new.test.tsx`, `app/routes/run-files.render.test.tsx`, `app/routes/run-detail.test.ts`, `app/routes/runs.test.tsx`, `app/routes/runs.preferences.test.tsx`, `app/components/run-summary-panel.test.tsx`.
- One Rust shared helper (`remote_run_summary_json` in `lib/crates/fabro-cli/tests/it/cmd/support.rs`) hand-wrote the principal JSON that matches `fabro_types::test_support::test_principal()`.

**Items deliberately left as-is:**
- The middleware `match &context.principal { Some(Principal::X) => ..., _ => auth_rejection(...) }` pattern is clean and readable; extracting a helper would obscure intent.
- The `fabro-workflow/src/test_support.rs` provenance uses `Principal::System { Engine }` deliberately (workflow runs originate from the engine), so it isn't a duplicate of `test_run_provenance()`.
- The `inspect.rs` snapshot embeds principal JSON inside an `fabro_snapshot!` stdout literal — by design, since snapshots assert exact textual output.
- `run_summary_round_trip.rs` hand-writes the principal JSON because the test asserts wire-shape parity.

### Fixes applied

1. Added `apps/fabro-web/app/lib/test-fixtures.ts` exporting `testPrincipal(): Principal`.
2. Updated all 8 frontend tests to import `testPrincipal` from that helper and removed their local definitions.
3. Replaced the hand-written `created_by` JSON in `lib/crates/fabro-cli/tests/it/cmd/support.rs::remote_run_summary_json` with `fabro_types::test_support::test_principal()` interpolated via `serde_json::json!`.

### Verification

- `cargo +nightly-2026-04-14 fmt --check --all` — passes
- `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings` — passes
- `cargo nextest run --workspace` — 6503 passed
- `cd apps/fabro-web && bun run typecheck` — passes
- `cd apps/fabro-web && bun test` — 508 passed