Review and cleanup complete.

What I changed:
- Renamed `fail_if_sandbox_provider_disabled` to `reject_run_if_sandbox_provider_disabled` in `lib/crates/fabro-server/src/server.rs` so the helper name clearly reflects that it mutates run state.
- Changed the launch-time policy log from `tracing::error!` to `tracing::warn!`, since rejecting a disabled sandbox provider is expected operator policy enforcement rather than an internal server error.

Review agent results:
- Code reuse: no issues found.
- Code quality: one issue found and fixed as above.
- Efficiency: no issues found.

Verification run:
- `cargo test -p fabro-config server_sandbox --quiet` ✅
- `cargo test -p fabro-install write_sandbox_settings_records --quiet` ✅
- `cargo test -p fabro-api --test server_settings_round_trip server_settings_json_matches_openapi_shape --quiet` ✅
- `cargo nextest run -p fabro-server sandbox_provider_policy` ✅
- `cargo +nightly-2026-04-14 fmt --check --all` ✅

Current working tree has one intended cleanup diff in `lib/crates/fabro-server/src/server.rs`.