Implemented the Automations backend API plan end-to-end.

### What changed

- Added new `fabro-automation` domain crate:
  - Automation/trigger IDs and validation.
  - TOML model with canonical file serialization.
  - SHA-256 revision hashing from exact TOML bytes.
  - File-backed async `AutomationStore` with atomic writes.

- Threaded automation metadata through runs:
  - `AutomationRef.trigger_id`
  - `RunSpec.automation`
  - `RunCreatedProps.automation`
  - workflow create/retry/fork/persistence paths
  - store projections into `Run.automation`

- Added OpenAPI contract and Rust API type reuse:
  - `/api/v1/automations`
  - `/api/v1/automations/{id}`
  - `/api/v1/automations/{id}/runs`
  - Reused `fabro-automation` Rust types where wire shape matches.
  - Regenerated TypeScript client under `lib/packages/fabro-api-client`.

- Wired server state:
  - Loads automation store from `dirname(active_config_path)/automations`.
  - Missing directory starts empty.
  - Malformed files fail startup.
  - Test support injects a fake automation run materializer.

- Added REST handlers:
  - CRUD with ETags/If-Match.
  - `422` domain validation for invalid automation/trigger shapes, including unknown future trigger shapes.
  - `409` for disabled automation or no enabled API trigger.
  - Run listing filtered by `Run.automation.id`.
  - API-triggered run creation with populated `automation.id`, `automation.name`, and `automation.trigger_id`.

- Added Git-based automation materializer:
  - Clones GitHub `owner/repo` targets.
  - Uses argv-based `git` commands, no shell string interpolation.
  - Uses non-interactive git and timeouts.
  - Resets origin to sanitized URL before manifest building.
  - Builds run manifests from resolved workflow selectors.

### Verification run

Passed:

```bash
cargo nextest run -p fabro-automation
cargo nextest run -p fabro-api
cargo nextest run -p fabro-types
cargo nextest run -p fabro-workflow operations::create
cargo nextest run -p fabro-store run_state
cargo nextest run -p fabro-server --features test-support automation_store
cargo nextest run -p fabro-server --features test-support automations
cargo nextest run -p fabro-server --features test-support openapi_conformance
cargo build -p fabro-api
cd lib/packages/fabro-api-client && bun run generate
cargo check -q --workspace
cargo +nightly-2026-04-14 fmt --check --all
cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings
git diff --check
```

Also confirmed:

```bash
git diff -- apps/fabro-web
```

is empty.

### Note

No UI routes/components, scheduler/background loop, or CLI commands were added. Two CLI test fixtures were minimally updated with `automation: None` because adding the required `RunSpec.automation` field otherwise breaks `--all-targets` compilation; no CLI behavior changed.

No commits were made.