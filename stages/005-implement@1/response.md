Implemented the plan in the requested two commits, in order:

1. `199c93c5e Remove anonymous principal actor`
   - Removed `Principal::Anonymous`.
   - Request auth now represents no actor as `None`.
   - HTTP logs use `principal_kind="none"` for absent principals.
   - Updated OpenAPI/generated TS client/frontend/tests accordingly.

2. `fffe671dd Make run provenance and creator total`
   - Made `Run.created_by`, `RunSpec.provenance`, `RunProvenance.subject`, and `run.created` provenance non-optional end-to-end.
   - Removed null/default/fallback provenance handling.
   - Updated create/retry/projection flows to require and propagate total provenance.
   - Added gated `fabro-types` test support helpers.
   - Added synthetic demo principal.
   - Updated OpenAPI and regenerated generated clients.
   - Updated frontend models/rendering/tests for total `created_by`.

Final repo state is clean.

Validation completed:
- `cargo build -p fabro-api`
- TypeScript API client generation with local JRE
- `cargo check -q --workspace --all-targets`
- `cargo +nightly-2026-04-14 fmt --check --all`
- `cargo +nightly-2026-04-14 clippy -q --workspace --all-targets -- -D warnings`
- `cargo build --workspace`
- `cargo nextest run --workspace --no-fail-fast`
- `cargo nextest run -p fabro-server`
- `cd apps/fabro-web && bun run typecheck && bun test && bun run build`
- `cd lib/packages/fabro-api-client && bun run typecheck`
- Final grep sweep for stale anonymous/null provenance traces; remaining hits are unrelated non-run-provenance `subject` fields.

Manual browser/server end-to-end checks from the plan were not run in this terminal environment.