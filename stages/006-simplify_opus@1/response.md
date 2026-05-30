All green. Summary of fixes applied based on the three reviews:

**Code reuse**
- **Consolidated 3 duplicate `test_run_provenance()` helpers** in `lib/crates/fabro-types/tests/{run_spec_serde,run_spec_methods,run_event_serde}.rs` by dual-listing `fabro-types` in its own dev-dependencies with the `test-support` feature, then importing the shared `fabro_types::test_support::test_run_provenance`. Removed ~36 lines of byte-identical copy-paste.
- **Replaced hardcoded principal JSON** in `lib/crates/fabro-cli/tests/it/cmd/support.rs` with `serde_json::to_value(test_principal())`, so future identity/auth-method changes won't drift.

**Code quality**
- **Switched `Principal::kind()` to `strum::IntoStaticStr`** per CLAUDE.md guidance ("Enum string/int conversions"). The hand-written match collapsed to `self.into()` and the variant→string map now lives in serde's `rename_all` only.
- **Used `Principal::user(...)` helper** in `lib/crates/fabro-workflow/src/operations/retry.rs` test module instead of constructing the `User(UserPrincipal { …, avatar_url: None })` struct variant directly; matches the rest of the workspace.

**Efficiency**
- **Removed `principal_without_log_unused_fields`** in `lib/crates/fabro-server/src/principal_middleware.rs`. The function cloned the entire `Principal` (3 owned `String` allocations) on every non-asset HTTP request just to overwrite `avatar_url`, but no HTTP log macro ever read `avatar_url`. Replaced with a single `context.principal.clone()`.

**Skipped (rationale recorded in task list)**
- Consolidating `fabro-workflow::event::test_support::user_principal` further — already a 4-line parameterized wrapper; cross-crate refactor cost outweighs the gain.
- Demo principal `LazyLock` + `.clone()` — the agent confirmed this is dominated by other allocations in `runs::summaries()`; only worth memoizing the whole vector if demo throughput ever shows up.
- Unifying the six `require_*` gate fallthroughs and the `emit_http_log!` two-branch macro — cosmetic; behavior is already correct and tested.

Verification: `cargo build --workspace`, `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings`, `cargo +nightly-2026-04-14 fmt --check --all`, `cargo nextest run -p fabro-types -p fabro-workflow -p fabro-server -p fabro-store -p fabro-cli` (3314 tests, 64 skipped, 0 failures), `bun run typecheck` — all green.