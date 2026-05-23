Review/cleanup complete.

Fixed:
- `projection_cache.rs`
  - Changed read-time overlay condition to match the design: only derive live timing when `summary.timing.is_none()`, so cached terminal `Conclusion.timing` always wins.
  - Moved live overlay work until after list date filtering, avoiding unnecessary stage timing aggregation for entries that will be filtered out.
  - Removed an unnecessary `mut`.

- `run_state.rs`
  - Added the missing sanity unit test: `live_run_timing_matches_conclusion_timing_at_conclusion_moment`.

Validated:
- `cargo nextest run -p fabro-store live_run_timing` ✅
- `cargo check -q -p fabro-store` ✅
- `cargo +nightly-2026-04-14 clippy -q -p fabro-store --all-targets -- -D warnings` ✅
- `cargo +nightly-2026-04-14 fmt --all` ✅

Working tree now has only these cleanup edits:
- `lib/crates/fabro-store/src/run_state.rs`
- `lib/crates/fabro-store/src/slate/projection_cache.rs`