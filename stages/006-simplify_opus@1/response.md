All six fixes landed, with net **-57 lines** even while adding three shared helpers and a constructor. Summary of cleanups applied based on the three reviewers' findings:

## Fixed

1. **Returned timing from `process_input_with_runtime` instead of stashing it on `Session`** (Agent 2 #1). Dropped `last_input_inference_duration` / `last_input_tool_duration` fields, the `last_input_timing()` getter, and the per-call reset prologue. Signature is now `-> (SessionInputTiming, Result<(), Error>)`. Eliminates the "must read immediately" implicit contract.

2. **Replaced the `record_inference_duration!` macro with a free function** `record_elapsed(&mut Option<Instant>, &mut Duration)` (Agent 1 #2, Agent 2 #2). Same 7 call sites, but type-checked, IDE-discoverable, and no macro hygiene.

3. **Made `run_store` required on `DetachedRunBootstrapGuard::arm()`** (Agent 1 #7, Agent 2 #3/#5). Dropped the `Option<RunStoreHandle>` field and the external setter pattern. Drop impl unconditionally builds the conclusion now.

4. **Extracted `emit_workflow_run_failed` helper** (Agent 1 #7, Agent 2 #4). Collapses the four near-identical "build conclusion → emit run.failed" sites in `start.rs` (terminal engine failure, both Drop guards, `persist_detached_failure`) into one shared function.

5. **Added `StageTiming::active_only(inference, tool)` constructor** (Agent 1 #1). Removed the local `active_stage_timing` helper in `handler/llm/api.rs` and unified `acp.rs` and `command.rs` on the same shape.

6. **Reverted `Box<BilledModelUsage>` in `CodergenResult::Text`** (Agent 1 #6, Agent 2 #6). Was the worst of both worlds — didn't equalise variant size, forced `.map(|usage| *usage)` ceremony at every call site. Used `#[allow(clippy::large_enum_variant, reason = "...")]` matching the established pattern in `event/events.rs` and `event/sink.rs`.

7. **Hoisted `test_usage` and `mark_run_running` to `test_support`** (Agent 1 #3/#4). `test_usage` was duplicated in 4 files (this PR was about to add a 5th); the 4th `mark_run_running` copy was right next to a pre-existing 3-event seed in test_support that didn't quite match. Both are now public helpers in `crate::test_support`.

## Skipped (with reason)

- **Make Drop guard tasks survive runtime shutdown** (Agent 3 #3) — flagged as a correctness-on-shutdown concern, not introduced by this PR; widening it would be a separate fix.
- **`wait_for_conclusion` polling timeout hardening** (Agent 2 #8) — minor CI flake risk; current 50ms budget is enough for unit tests.
- **`Candidate::timing` zero on LLM-failure heuristic fallback** (Agent 2 #9, Agent 3 implicit) — agent itself noted this is acceptable; capturing wasted inference on errors would require threading Instants through the fan-in evaluator.
- **Helper for the 3-line failed-outcome decoration in agent.rs/prompt.rs** (Agent 2 #7/#10) — marginal benefit not worth the indirection.

## Verification

- `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings` clean
- `cargo +nightly-2026-04-14 fmt --check --all` clean
- `cargo nextest run -p fabro-workflow -p fabro-agent -p fabro-types -p fabro-core` — all 3592 tests pass (1175 + 1844 + 573 across the affected crates).