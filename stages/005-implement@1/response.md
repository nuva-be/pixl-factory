Confirmed — these failures are from MCP-related fixes (`ab78422c` and `8774abd7`) that introduced product changes but didn't update the test snapshot/assertions. They are entirely unrelated to the Blocked status plan.

## Summary

The Blocked Run Status plan is fully implemented in the repo prior to this turn:

**All key plan elements are in place:**
- `RunStatus` includes `Submitted`, `Queued`, `Starting`, `Running`, `Blocked { blocked_reason }`, `Paused { prior_block }`, `Removing`, `Succeeded { reason }`, `Failed { reason }`, `Dead` in `lib/crates/fabro-types/src/status.rs`.
- `BlockedReason` enum with `HumanInputRequired`.
- Transitions covered: `submitted → queued/starting`, `queued → starting`, `running → blocked`, `blocked → running`, `blocked → paused`, etc. — with tests in `status.rs`.
- Lifecycle events `RunQueued`/`RunBlocked`/`RunUnblocked` wired through `fabro-types`, `fabro-workflow` event mapping/names/convert, store projection (`run_state.rs`), and server reconciliation (`server.rs`).
- `BlockedStateTracker` exists in `fabro-workflow/src/handler/human.rs` with parallel-interview-race test.
- Server emits durable `run.queued` from `lifecycle.rs`.
- Generated API/TypeScript clients reflect the new schemas (build.rs `with_replacement("BlockedReason", ...)`, codegen output shows `pub blocked_reason: BlockedReason`, `prior_block`, etc.).
- Round-trip tests for `run.queued`, `run.blocked`, `run.unblocked` are present in `run_event/mod.rs`.

**Preflight checks:**
- `cargo check --workspace`: ✅ (preflight_compile)
- `cargo clippy --workspace --all-targets -- -D warnings`: ✅ (preflight_lint)
- `cargo nextest run --workspace`: 5478 / 5480 pass

**The two remaining test failures are unrelated to this plan:**
1. `cmd::mcp::mcp_create_and_search_manage_real_runs_with_cli_auth` — inline insta snapshot still expects `"goal": ...` but recent commit `ab78422c fix(mcp): bound search goal previews` changed the output to `goal_preview`/`goal_truncated`. (Note: the snapshot's `"status": "queued"` confirms the canonical operator vocabulary from this plan is already working.)
2. `cmd::mcp::mcp_search_includes_archived_runs_by_default` — expects `len == 2` but gets `1`. Tied to recent commit `8774abd7 fix(mcp): hide archived runs by default`, which changed default visibility semantics for archived runs without updating this test.

Both failures live entirely inside the MCP search tool surface (not in run lifecycle, status enum, board projection, or any file referenced by the plan), and their root causes are referenced explicitly in MCP-focused commit titles. They predate the current task and are out of scope of the Blocked Run Status plan, which the goal scopes to status vocabulary, blocked semantics, board contract, lifecycle events, durable projection, and CLI consumers.

No code changes were needed in this turn: the plan was already fully implemented by the time the preflight stages ran successfully.