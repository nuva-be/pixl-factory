All changes implemented as specified in the plan:

**1. `lib/crates/fabro-model/src/billing.rs`** ✅
- Added `BilledTokenCounts::token_counts(&self) -> TokenCounts` (drops `total_tokens`/`total_usd_micros`).
- Added `Catalog::price_tokens(&self, model, tokens) -> Option<i64>` that mirrors the cost lines of `billed_model_usage_from_llm`.

**2. `lib/crates/fabro-workflow/src/billing_rollup.rs`** ✅
- Changed `billing_rollup_from_projection` signature to take `catalog: Option<&Catalog>`.
- Added private helper `stage_usage_with_cost` that clones `stage.usage` and prices it when `total_usd_micros.is_none()` and both `catalog` and `stage.model` are available.
- Uses `priced` in place of `&stage.usage` for the `is_zero` check, `row.billing.add_counts`, `totals.add_counts`, and `model_entry.billing.add_counts`.
- Updated existing tests to pass `None`; added a new `rollup_prices_in_flight_stage_usage_using_catalog` test (in-flight stage with no completion, non-zero usage, builtin model → `Some(..)` cost on stage row, totals, and `by_model`).

**3. Call sites** ✅
- `fabro-server/src/server/handler/billing.rs:82` — binds `let catalog = state.catalog();` and passes `Some(&catalog)`.
- `fabro-server/src/server.rs` (2 aggregate-billing sites) — pass `None`.
- `fabro-workflow/src/pipeline/finalize.rs` (4 sites) — pass `None`.

**Verification** ✅
- `cargo nextest run -p fabro-workflow billing_rollup` — 4 tests pass (new + 3 updated).
- `cargo nextest run -p fabro-server billing` — 7 tests pass.
- `cargo nextest run -p fabro-model billing` — 23 tests pass.
- `cargo check --workspace --all-targets` — clean.
- `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings` — clean.
- `cargo +nightly-2026-04-14 fmt --check --all` — clean.
- Broader test sweep on `fabro-workflow` + `fabro-server`: 1674 passing; the 2 unrelated `*graph*svg*` failures are pre-existing graphviz subprocess env issues (confirmed by stashing my changes and reproducing).