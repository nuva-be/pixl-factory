Implemented the full IP allowlisting removal refactor.

Summary:
- Removed inbound source-IP allowlist config from dense settings and sparse config layers:
  - `[server.ip_allowlist]`
  - `[server.integrations.github.webhooks.ip_allowlist]`
  - `IpAllowEntry` and related settings/types.
- Deleted server runtime enforcement:
  - Removed `ip_allowlist` middleware/module.
  - Removed `IpAllowlistConfig`, client-IP extraction, trusted proxy handling, and GitHub `/meta` hook range resolution.
  - Simplified router/startup wiring and removed `ConnectInfo` serving requirements that only existed for allowlisting.
- Preserved unrelated allowlists:
  - GitHub username allowlists remain.
  - Sandbox/Daytona CIDR allow lists remain.
- Updated OpenAPI and generated clients:
  - Removed allowlist schemas/fields from `fabro-api.yaml`.
  - Regenerated Rust API and TypeScript API client.
  - Deleted stale generated TS allowlist model files.
- Updated web UI:
  - Removed Settings > Security network allowlist display.
  - Adjusted Security page/nav copy.
- Updated tests/docs:
  - Added unknown-field tests proving stale IP allowlist config is now rejected.
  - Added settings API assertions that removed fields are absent.
  - Updated security docs and changelog to direct operators to upstream network controls.
- Updated dependency graph:
  - Removed direct `ipnet` deps from `fabro-types` and `fabro-server`; retained transitive/use in `fabro-config` for sandbox CIDR validation.

Red/green TDD:
- Added stale-config rejection tests and verified they failed before schema removal, then passed after removal.

Validation run:
- `cargo build -p fabro-api`
- `cd lib/packages/fabro-api-client && bun run generate`
- `cargo build -p fabro-cli --bin fabro`
- `cargo nextest run -p fabro-config -p fabro-types -p fabro-api -p fabro-server --no-fail-fast`
- `cargo +nightly-2026-04-14 fmt --check --all`
- `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings`
- `git diff --check`
- Stale-symbol search confirmed remaining `ip_allowlist`/IP allowlist references are only intentional docs/tests/assertions.

No commits were created.