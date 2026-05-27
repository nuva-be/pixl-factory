All checks pass. Summary of cleanup:

**Fixes applied** based on the three review agents' findings:

1. **Consolidated triplicate `test_run_provenance()`** (Code Reuse review): Added `engine_run_provenance()` to `fabro-types::test_support`, dual-listed `fabro-types` in its own dev-deps with `test-support` feature, and deleted the three byte-identical local helpers in `fabro-types/tests/{run_event_serde,run_spec_methods,run_spec_serde}.rs`.

2. **Split HTTP log match arm** (Code Quality review): In `server.rs`, separated `None => emit_http_log!($level)` from `Some(Principal::Agent { .. } | Principal::System { .. })` so the principal-absent case and stateless-principal case have distinct arms.

3. **Inlined `create_run_input` helper** (Code Quality review): The helper in `run_manifest.rs` had a single caller that immediately patched two of its fields. Inlined the full `CreateRunInput` struct literal into the handler and removed the helper plus its now-unused imports.

**Findings skipped** with rationale:
- `Option<Principal>` → `RequestAuth` sum type: too invasive; the existing shape is readable.
- `DEMO_PRINCIPAL` as `LazyLock`: reviewers disagreed; current form is fine since `Principal::user` is not const-constructible.
- Hand-rolled JSON principals in insta snapshots: intentional — snapshots assert the literal wire shape.
- Cross-crate `test_principal(login)` parameterization: would require feature-gating `fabro-workflow::test_support`, out of scope.
- TS `test-principal.ts` location: consistent with `app/lib/test-utils.tsx` convention.
- Per-event `Principal` clone in `build_summary` (Efficiency review): pre-existing pattern, flagged as follow-up only.

**Verification**: `cargo build --workspace`, `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings`, `cargo +nightly-2026-04-14 fmt --check --all`, `cargo nextest run --workspace` (6469 passed), and `bun run typecheck` all green. The 14 TS test failures are pre-existing on baseline (confirmed via `git stash`).