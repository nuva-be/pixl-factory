All frontend tests pass.

## Summary

Reviewed both commits with three parallel agents (reuse, quality, efficiency). Findings triaged and applied:

**Fixed:**
- **Inconsistent indentation** in `apps/fabro-web/app/routes/run-files.render.test.tsx` (`created_by` fixture had under-indented inner keys) — re-indented to match surrounding nesting.
- **Inconsistent `<EmptyValue />` indentation** in `apps/fabro-web/app/components/run-summary-panel.tsx` — the two cells touched by the diff had been fixed, but three other cells (Sandbox, Cost, Artifacts) still had the same under-indented `<EmptyValue />`. Made all five consistent.

**Skipped (with rationale):**
- *Duplicated `test_run_provenance` in `fabro-workflow/src/test_support.rs`* — intentionally uses `Principal::System { Engine }` rather than a user, since workflow-engine tests model the engine itself as the creator. Not a true duplicate.
- *Unifying `test_principal()` across crates* — existing per-crate helpers use the `https://github.com` issuer with `AuthMethod::Github`, while the new `fabro_types::test_support::test_principal()` deliberately uses `fabro:test`/`DevToken`. Consolidating would require widening the helper API beyond the scope of this PR.
- *Match arm `None | Some(Principal::Agent { .. } | Principal::System { .. })` in `server.rs`* — exhaustive, concise, and exhaustiveness-checked over `Principal` variants. Splitting would add noise.
- *Stringly-typed `"none"` for `principal_kind`* — used in exactly one place; a named constant would be over-engineering.
- *`<EmptyValue />` fallback on "Created by"* — still reachable when `run` is undefined but not loading (e.g. error states), so not dead code.
- *Per-request `DEMO_PRINCIPAL.clone()` allocations in demo summaries* — pre-existing pattern, the `LazyLock` already eliminates the per-request `IdpIdentity::new` validation cost. Promoting summaries to a static `Vec<Run>` is a larger refactor unrelated to this PR's scope.

Frontend `typecheck` and `bun test` (508 pass) both green.