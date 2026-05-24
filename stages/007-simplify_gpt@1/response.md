Review completed. I ran the requested parallel subagent-style review pass and applied a small cleanup set:

- Changed `delete_run_internal` and `delete_run_sandbox_resource` to take `&AppState` instead of `&Arc<AppState>` to keep the internal delete flow from depending on ownership/container details.
- Updated call sites in run delete, prune, batch delete, and tests accordingly.
- Imported `DeleteRunOutcome` directly in `handler/runs.rs` to avoid repeated fully-qualified paths.
- Renamed the shared batch limit constant from `MAX_BATCH_RUN_LIFECYCLE_IDS` to `MAX_BATCH_RUN_IDS` now that it is used for delete as well as archive/unarchive.
- Ran Rust formatting.

Verification passed:

- `cargo nextest run -p fabro-server batch_delete delete_run prune_runs`
- `cd apps/fabro-web && bun test app/lib/run-actions.test.ts`
- `cargo +nightly-2026-04-14 fmt --check --all`
- `cargo build -p fabro-api`
- `cd apps/fabro-web && bun run typecheck`

Working tree now has only the small cleanup edits in the server files.