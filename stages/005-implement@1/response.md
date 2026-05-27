Implemented CLI variable management end-to-end.

Summary:
- Added `fabro-client` wrappers for variables API:
  - `list_variables`
  - `get_variable`
  - `create_variable`
  - `update_variable`
  - `delete_variable`
- Added `fabro variable` CLI namespace:
  - `fabro variable list` / `ls`
  - `fabro variable get NAME`
  - `fabro variable set NAME [VALUE]`
  - `fabro variable rm NAME`
- Implemented behavior:
  - Plain `list` table includes `NAME`, `VALUE`, `UPDATED`
  - Plain `get` prints raw value
  - JSON output returns full variable objects
  - `set` is an upsert and supports `--description`
  - `set` allows explicit empty values and empty `--value-stdin`
  - `rm` JSON returns `{ "name": NAME }`
- Added CLI integration test coverage and `TestContext::variable()`.
- Updated root help snapshots and generated CLI reference docs.
- Updated workflow variables docs to explain server-managed `{{ vars.NAME }}` and distinguish variables from secrets.

TDD note:
- Added variable CLI tests first and confirmed red state with `cargo nextest run -p fabro-cli -- variable::help` failing because `variable` was not yet implemented.

Validation run:
- `cargo nextest run -p fabro-cli -- variable` ✅
- `cargo nextest run -p fabro-cli -- fabro` ✅
- `cargo dev docs refresh` ✅
- `cargo dev docs check` ✅
- `cargo +nightly-2026-04-14 fmt --check --all` ✅
- `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings` ✅
- `cargo check -q --workspace` ✅
- `git diff --check` ✅

Note: `cargo insta pending-snapshots` was unavailable because `cargo-insta` is not installed in the environment; checked manually and found no pending `*.snap.new` / `*.snap.pending` files.