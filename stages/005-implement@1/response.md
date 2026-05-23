Implemented output schema validation end-to-end.

Summary:
- Added `output_schema` / `output_retries` node attrs.
- Added `output_schema` static/file reference handling and inlining.
- Added `handler::structured_output` for:
  - routing/custom schema parsing
  - balanced JSON extraction
  - JSON Schema validation
  - repair prompts
  - routing/custom context application
- Integrated strict routing/custom output into agent and prompt handlers.
- Added same-context repair for:
  - prompt API calls via mutable message history
  - agent API sessions via the same live `Session`
- Added ACP guardrail:
  - `output_schema is not supported with backend="acp" in this release`
  - test verifies no process launch
- Added custom output storage at `output.{node_id}`.
- Updated docs for routing/custom schemas, `output_retries`, repair behavior, ACP limitation.

Validation run:
- `cargo nextest run -p fabro-types -p fabro-workflow graph:: file_inlining static_reference` ✅
- `cargo nextest run -p fabro-workflow` ✅
- `cargo +nightly-2026-04-14 fmt --check --all` ✅
- `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings` ✅
- `cargo insta pending-snapshots` unavailable in this environment (`cargo-insta` not installed); checked for `**/*.snap.new` and found none.