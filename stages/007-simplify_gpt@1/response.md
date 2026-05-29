Implemented the simplify/review cleanup.

Changed:
- `lib/crates/fabro-automation/src/model.rs`
  - Added the missing second sentence to `Automation::enabled_api_trigger()` docs.
  - Renamed the focused no-compatibility test to `persisted_toml_rejects_legacy_top_level_enabled`.

Validation run:
- `cargo nextest run -p fabro-automation` ✅
- `cargo nextest run -p fabro-api --test automation_round_trip` ✅
  - Note: the plan’s `cargo nextest run -p fabro-api automation_round_trip` selector matched zero tests, so I reran with the explicit integration test target.
- `cargo nextest run -p fabro-server --features test-support --test it automations` ✅
  - Note: the server integration test target requires `test-support`.
- `cargo build -p fabro-api` ✅
- `cd lib/packages/fabro-api-client && bun run typecheck` ✅
- `cd apps/fabro-web && bun run typecheck` ✅
- Leftover master-gate search ✅
  - Only trigger-level `enabled` hits remain, plus the expected `enabled_api_trigger()` handler call.

Working tree has one small source modification:
- `lib/crates/fabro-automation/src/model.rs`