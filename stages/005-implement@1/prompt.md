Goal: # Slack Run Lifecycle Notifications

## Summary

Add Slack notifications for `run.started`, `run.completed`, and `run.failed` using the existing `[run.notifications]` namespace as the per-run/per-workflow source of truth. Keep Slack interviews as separate behavior sharing the same Slack connection, and do not move notification settings to server config.

## Key Changes

- Treat configured routes like this as active runtime subscriptions:

  ```toml
  [run.notifications.deploys]
  enabled = true
  provider = "slack"
  events = ["run.started", "run.completed", "run.failed"]

  [run.notifications.deploys.slack]
  channel = "#deploys"
  ```

- Require `[run.notifications.<name>.slack].channel` for lifecycle Slack notifications; skip and log a warning if missing or unresolved.
- Start `SlackService` when Slack credentials are present, even if `server.integrations.slack.default_channel` is absent. Keep `default_channel` only for the existing interview path.
- Keep interview messages and lifecycle notifications independent:
  - interviews keep `posted_messages` and `thread_registry` behavior
  - lifecycle notifications are fire-and-forget and never accept answers or update messages

## Implementation

- In `lib/crates/fabro-server/src/server.rs`, extend `SlackService::handle_event`:
  - Existing interview event handling remains unchanged.
  - Add a lifecycle path for `RunStarted`, `RunCompleted`, and `RunFailed`.
  - For lifecycle events, load the cached run projection, inspect `projection.spec.settings.run.notifications`, select enabled Slack routes whose `events` contains the current event name, resolve each route channel, and post once per route.
- In `lib/crates/fabro-slack/src/blocks.rs`, add lifecycle-specific Block Kit builders separate from interview builders.
  - Include run ID, Fabro run link when available, workflow label, result when applicable, duration when applicable, and PR info when available.
  - Use existing Slack escaping/truncation patterns for all untrusted text.
- Derive fields as follows:
  - workflow: workflow name, then workflow slug, then graph name, then `run.started` event name
  - result: completed status/reason or failed reason/message
  - duration: `RunTiming.wall_time_ms` from completed/failed events, formatted compactly
  - PR: latest prior `pull_request.created` event for number/title/link; if unavailable, fall back to projection `PullRequestLink` with number/link only
- Update public docs for Slack and run configuration to document `[run.notifications]` and remove or qualify the "interviews only" limitation.

## Test Plan

- Add `fabro-slack` unit tests for lifecycle block rendering, escaping, truncation, run links, duration formatting, failed/completed variants, and optional PR fields.
- Add server tests with a fake Slack endpoint/client path proving:
  - `run.started` posts for a matching enabled route
  - `run.completed` includes result and duration
  - `run.failed` includes failure result/message and duration
  - non-matching events and disabled routes do not post
  - missing lifecycle route channel is skipped and does not affect other routes
  - prior `pull_request.created` adds PR number/title/link
  - interview Slack messages still use interview state and are not mixed with lifecycle notification state

## Assumptions

- Lifecycle Slack notifications are opt-in only through `[run.notifications]`.
- `provider = "slack"` is required; other providers remain parsed but unsupported.
- Route-level Slack channel is required for lifecycle notifications.
- Slack post failures are logged and never fail, block, or alter the run.
- No GitHub API call is made during notification delivery; PR title comes only from existing run events.


## Completed stages
- **toolchain**: succeeded
  - Script: `command -v cargo >/dev/null || { curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && sudo ln -sf $HOME/.cargo/bin/* /usr/local/bin/; }; cargo --version 2>&1`
  - Output:
    ```
    cargo 1.95.0 (f2d3ce0bd 2026-03-21)
    ```
- **preflight_compile**: succeeded
  - Script: `cargo check -q --workspace 2>&1`
  - Output: (empty)
- **preflight_lint**: succeeded
  - Script: `cargo +nightly-2026-04-14 clippy -q --workspace --all-targets -- -D warnings 2>&1`
  - Output: (empty)


Read the plan file referenced in the goal and implement every step. Make all the code changes described in the plan. Use red/green TDD.