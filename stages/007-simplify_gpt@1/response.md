Reviewed the feature diff against `origin/main` and completed cleanup.

Fixed:
- Removed duplicated batch command logic between:
  - `approve` / `deny`
  - `archive` / `unarchive`
- Added shared helper in `lib/crates/fabro-cli/src/commands/runs/mod.rs`:
  - resolves each selector
  - attempts all runs
  - prints short IDs in text mode
  - emits the common JSON `{ key: [...], errors: [...] }` shape
  - returns a non-zero error after processing all failures
- Simplified:
  - `lib/crates/fabro-cli/src/commands/runs/approval.rs`
  - `lib/crates/fabro-cli/src/commands/runs/archive.rs`

Validation run:
- `cargo +nightly-2026-04-14 fmt --all`
- `cargo check -q -p fabro-cli`
- `cargo nextest run -p fabro-cli approve deny archive unarchive`
- `cargo nextest run -p fabro-tool interact`
- `cargo nextest run -p fabro-server run_tools_worker_cannot_call_user_only_non_mcp_routes`
- `cargo +nightly-2026-04-14 clippy -q -p fabro-cli --all-targets -- -D warnings`

All passed.