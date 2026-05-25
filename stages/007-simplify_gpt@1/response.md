Reviewed the branch diff vs merge base and ran the parallel review pass. I found and fixed one cleanup issue:

- Restored `Session::process_input_with_runtime` to its original public `Result<(), Error>` shape instead of returning a tuple.
- Added the requested `Session::last_input_timing()` getter and `last_input_timing` state on `Session`.
- Updated workflow callers to read timing through the getter after `process_input_with_runtime(...)`.
- Updated the agent timing unit test accordingly.

Validation passed:

- `cargo check -q -p fabro-agent -p fabro-workflow`
- `cargo nextest run -p fabro-agent -p fabro-workflow`
- `cargo nextest run -p fabro-store -p fabro-core`
- `cargo +nightly-2026-04-14 fmt --check --all`
- `cargo +nightly-2026-04-14 clippy -q -p fabro-agent -p fabro-workflow --all-targets -- -D warnings`

Working tree now has the cleanup edits in:

- `lib/crates/fabro-agent/src/session.rs`
- `lib/crates/fabro-workflow/src/handler/llm/api.rs`