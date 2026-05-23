All checks pass. The previous verify failure was a flaky test (`system_df_summarizes_runs_and_logs`) in fabro-cli — a pre-existing issue unrelated to my legacy sandbox migration changes:

- The test passes in isolation (verified).
- The full workspace test suite passed: **6259/6259 tests passed**.
- The failure mode (`projection cache sequence gap`) was in `fabro-store`/`fabro-server` event ingestion — code I did not touch.
- My changes are confined to `lib/crates/fabro-config/` and `docs/public/`.

Verification summary:
- `cargo +nightly-2026-04-14 fmt --check --all` — pass
- `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings` — pass
- `cargo nextest run --workspace --status-level slow --profile ci` — 6259 passed, 181 skipped
- `cargo dev docs check` — pass
- `bun install --frozen-lockfile` — pass
- `apps/fabro-web` typecheck + test — 446/446 pass
- `lib/packages/fabro-api-client` typecheck — pass

No code changes were needed; the previous failure was an unrelated flake that did not reproduce.