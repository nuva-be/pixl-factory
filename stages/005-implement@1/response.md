Implemented run approval controls for MCP and CLI.

Summary:
- Added `fabro_client::Client::approve_run` and `deny_run`.
- Extended `FabroToolBackend` and `ClientBackend` with approval methods.
- Extended `fabro_run_interact` with:
  - `approve`
  - `deny`
  - optional `reason`
  - reason trimming / blank-to-`None`
  - summary-returning dispatch.
- Updated MCP tool descriptions and schema coverage.
- Blocked workflow-agent `fabro_tools` from approving/denying runs with an immediate explicit `ToolError`.
- Added top-level CLI commands:
  - `fabro approve <RUNS>...`
  - `fabro deny [--reason <REASON>] <RUNS>...`
- Added batch JSON/text behavior and partial-error handling.
- Updated CLI/MCP docs and verified generated CLI docs are current.
- Extended server auth regression to include `/runs/{id}/deny`.

Validation run:
- Red step: `cargo nextest run -p fabro-tool interact` failed before implementation as expected.
- `cargo nextest run -p fabro-tool interact`
- `cargo nextest run -p fabro-cli approve deny fabro::help stdio_server_initializes_and_lists_run_tools mcp_interact`
- `cargo nextest run -p fabro-workflow agent_run_interact_rejects_approval_actions_before_backend_dispatch`
- `cargo nextest run -p fabro-server run_tools_worker_cannot_call_user_only_non_mcp_routes`
- `cargo nextest run -p fabro-tool -p fabro-cli -p fabro-server`
- `cargo dev docs check`
- `cargo +nightly-2026-04-14 fmt --check --all`
- `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings`
- `git diff --check`

Note: `cargo insta pending-snapshots` could not be run because `cargo-insta` is not installed in this environment, but the affected inline snapshot tests passed.