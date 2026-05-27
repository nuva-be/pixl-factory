Reviewed the branch diff and applied one cleanup.

Fixed:
- Removed the extra `engine_run_provenance()` test helper from `fabro-types::test_support`.
- Updated affected `fabro-types` serde/method tests to use the canonical requested `test_run_provenance()` helper instead.
- This keeps test-support minimal and aligned with the plan’s intended API: `test_principal()` + `test_run_provenance()`.

Validation run:
- `cargo +nightly-2026-04-14 fmt --check --all` ✅
- `cargo check -q --workspace` ✅
- `cd apps/fabro-web && bun run typecheck` ✅
- `cargo nextest run -p fabro-types` ✅
- `cargo nextest run -p fabro-server principal_middleware` ✅

Working tree now has only the cleanup edits in:
- `lib/crates/fabro-types/src/test_support.rs`
- `lib/crates/fabro-types/tests/run_event_serde.rs`
- `lib/crates/fabro-types/tests/run_spec_methods.rs`
- `lib/crates/fabro-types/tests/run_spec_serde.rs`