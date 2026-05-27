Implemented the two-commit cleanup and committed both changes:

- `d15d3d033 Remove anonymous principal variant`
- `5d14b2ed0 Make run provenance total`

Key outcomes:
- `Principal` is actor-only; missing auth is represented as `Option<Principal>` and HTTP logs use `principal_kind="none"`.
- `Run.created_by`, `RunSpec.provenance`, `RunProvenance.subject`, and `run.created` provenance are now total across Rust, OpenAPI, generated TS client, frontend, projections, create/retry flows, demo mode, and tests.
- Added `fabro-types` test-support helpers and frontend `testPrincipal` fixture.
- Regenerated/validated Rust and TypeScript API client outputs.
- Working tree is clean.

Validation run:
- `cargo +nightly-2026-04-14 fmt --check --all`
- `cargo check -q --workspace`
- `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace`
- `cargo nextest run --workspace`
- `cargo nextest run -p fabro-server`
- `cd lib/packages/fabro-api-client && bun run typecheck`
- `cd apps/fabro-web && bun run typecheck`
- `cd apps/fabro-web && bun run test`
- `cd apps/fabro-web && bun run build`
- Final sweeps for anonymous principals and nullable creator/provenance patterns

Note: direct `cd apps/fabro-web && bun test` without the package script failed due the suite’s non-isolated shared Axios adapter state; the repo’s configured test script (`bun run test`, which runs `bun test --isolate`) passes. Manual browser/server E2E checks were not run in this terminal session.