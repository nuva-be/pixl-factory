Implemented Slack run lifecycle notifications.

Summary:
- Added lifecycle Block Kit builders in `fabro-slack` for `run.started`, `run.completed`, and `run.failed`.
- Extended `SlackService` to:
  - keep interview behavior/state unchanged,
  - load cached run projections,
  - inspect `[run.notifications]`,
  - select enabled Slack routes matching the current event,
  - resolve required route channels,
  - post lifecycle notifications fire-and-forget,
  - log and skip missing/unresolved channels and post failures.
- Changed Slack service startup so credentials create a service even without `server.integrations.slack.default_channel`; `default_channel` is now only used for interview prompts.
- Added PR detail resolution from latest prior `pull_request.created`, with projection `PullRequestLink` fallback.
- Added fake Slack endpoint/server tests covering the requested routing and separation cases.
- Updated public Slack, run configuration, and server configuration docs.

Validation run:
- `cargo nextest run -p fabro-slack` ✅
- `cargo nextest run -p fabro-server slack_` ✅
- `cargo check -q --workspace` ✅
- `cargo +nightly-2026-04-14 fmt --check --all` ✅
- `cargo +nightly-2026-04-14 clippy -q --workspace --all-targets -- -D warnings` ✅
- `git diff --check` ✅

Note: a full `cargo test -q -p fabro-server --lib` / `cargo nextest run -p fabro-server` still hits existing unrelated graph-render subprocess failures in this environment (`get_graph_returns_svg` invokes the test binary and gets “running 0 tests” instead of SVG). Focused Slack tests pass.