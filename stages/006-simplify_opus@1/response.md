All checks green. Summary of cleanup:

**Fixed:**

1. **Duplicate `TEST_PRINCIPAL` literal across 8 frontend test files** — extracted to `apps/fabro-web/app/lib/test-fixtures.ts` (typed as `Principal` from the API client). All 8 test files now import it instead of redeclaring.

2. **Stale assertion + misleading test name** in `run-summary-panel.test.tsx` — removed the `expect(... "Created by"))).toBe("Ttest")` line that no longer fit the "shows unavailable copy" semantics, and dropped the leftover "optional" wording from the test title.

3. **Leftover IIFE wrapper** in `run-table-row.tsx` — hoisted `principalDisplay(run.createdBy)` to a `const createdByDisplay = …` at the top of the component and inlined the `<Tooltip>` directly. Removed the no-longer-needed self-invoking-function shape that existed only for the old `run.createdBy && …` guard.

4. **`run_created_properties` compat-shim** in `fabro-store/src/run_state.rs` — removed. The eight `test_raw_event(… "run.created" …)` JSON fixtures now include `"provenance": test_support::test_run_provenance()` explicitly, so the conditional injection of provenance no longer hides untouched fixtures.

**Skipped (false positive / out-of-scope):**

- The duplicate local `test_run_provenance()` in `fabro-workflow/src/test_support.rs` looked redundant, but the workflow crate exposes `pub mod test_support;` *unconditionally* while `fabro_types::test_support` is feature-gated and only enabled as a dev-dependency. Replacing the local copy broke the library build, so the duplicate stays.
- Demo `LazyLock<Principal>` — correct as-is.
- `Option<Principal>` in `principal_middleware.rs` — intentional per the plan ("missing/invalid auth is represented as absent request principal").

**Verified:** `cargo check --workspace`, `cargo clippy --workspace --all-targets -D warnings`, `cargo fmt --check`, `cargo nextest run --workspace` (6500 pass), `bun run typecheck`, `bun test` (508 pass).