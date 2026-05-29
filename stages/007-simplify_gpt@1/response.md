Reviewed the branch diff and applied one cleanup:

- Deduplicated shared `If-Match` parsing, ETag quoting, and JSON-with-ETag response construction between Automations and Environments.
- Added shared helpers in `lib/crates/fabro-server/src/server/handler/mod.rs`.
- Updated:
  - `lib/crates/fabro-server/src/server/handler/automations.rs`
  - `lib/crates/fabro-server/src/server/handler/environments.rs`

Validation run:

- `cargo +nightly-2026-04-14 fmt --check --all`
- `cargo check -q -p fabro-server -p fabro-api`
- `cargo nextest run -p fabro-server --features test-support --test it api::environments`
- `cargo +nightly-2026-04-14 clippy -q -p fabro-server --features test-support --test it -- -D warnings`

All passed.