All verification steps passed:

- `git merge --no-edit origin/main` (after setting git identity)
- `cargo +nightly-2026-04-14 fmt --all` and `fmt --check --all`
- `cargo dev docs refresh` / `docs check`
- Forbidden-symbol grep
- `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile ci` — 6259 passed, 181 skipped (one initial flake in `artifact_cp_empty_run_reports_no_artifacts` due to event ordering under parallel load; passed individually and on the full re-run)
- `bun install --frozen-lockfile`, `bun run typecheck` (fabro-web), `bun run test` (fabro-web), `bun run typecheck` (fabro-api-client)
- `cargo dev build -- -p fabro-cli --release`

The original verify failure was purely the missing git committer identity; once configured, the merge from `origin/main` (which brought in commit `dbe3e3966` "Migrate sandbox config to named environments" plus several unrelated changes) integrated cleanly and the full pipeline is green.