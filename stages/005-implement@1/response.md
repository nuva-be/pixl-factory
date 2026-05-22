All changes are in place. Summary of what was done:

**Task 1** — Already completed in prior commits (types/layers/resolver + tests).

**Task 2** — Completed the remaining gaps:
- Added `FABRO_WORKER_AGENT_RUN_TOOLS` constant to `fabro-static/src/env_vars.rs` (including the alphabetized test list).
- Updated `worker_command` in `fabro-server/src/server.rs` to clear the env var and conditionally set it to `"true"` when `agent_fabro_tools_enabled` is set.
- Extended both server tests (`worker_command_default_token_omits_agent_run_tools_scope` and `worker_command_opt_in_token_includes_agent_run_tools_scope`) to assert the env var behavior. The default-case assertion uses `EnvOverride::Unchanged` rather than `Removed` because `apply_worker_env` calls `Command::env_clear()` first, which makes a subsequent `env_remove` a true no-op in `get_envs()` — the conceptual invariant ("absent from worker env") still holds.

**Task 3** — Switched the CLI worker to use the env var as the gate:
- Added `use fabro_static::EnvVars;` import.
- Replaced `if run_spec.settings.run.agent.fabro_tools` with `if fabro_run_tools_enabled_from_env(process_env_var(EnvVars::FABRO_WORKER_AGENT_RUN_TOOLS).as_deref())`.
- Added the `fabro_run_tools_enabled_from_env` helper that only treats the exact string `"true"` as enabled.
- Added the `fabro_run_tools_enabled_env_requires_true` unit test in the existing `mod tests`.

**Task 4** — Docs already updated in prior commits.

**Task 5** — Verification:
- `cargo nextest run -p fabro-config` (197 tests pass)
- `cargo nextest run -p fabro-server` (583 tests pass)
- `cargo nextest run -p fabro-cli` (906 tests pass)
- `cargo +nightly-2026-04-14 fmt --check --all` passes
- `cargo +nightly-2026-04-14 clippy -p fabro-types -p fabro-config -p fabro-static -p fabro-server -p fabro-cli -p fabro-dev --all-targets -- -D warnings` passes
- `cargo dev docs check` reports docs up to date

Final invariants are honored:
- Default run → `run.agent.fabro_tools == false`, JWT scope is just `run:worker`, `FABRO_WORKER_AGENT_RUN_TOOLS` is absent from the worker env, and `StartServices.fabro_run_tools == None`.
- Opted-in run → `run.agent.fabro_tools == true`, JWT scope is `run:worker agent:run_tools`, `FABRO_WORKER_AGENT_RUN_TOOLS == "true"` is set in the worker env, and the env-gated branch in `runner.rs` builds `FabroRunToolServices`.