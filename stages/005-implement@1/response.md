Implemented/verified the batch run archive actions. The current branch already contained the requested feature implementation, so no additional tracked code changes were needed.

Verified coverage includes:

- OpenAPI batch endpoints/schemas:
  - `POST /api/v1/runs/archive`
  - `POST /api/v1/runs/unarchive`
  - batch request/response/result/summary schemas
- Server batch handlers:
  - `RequiredUser` auth
  - bounded validation
  - fail-soft per-run results
  - existing single-run archive/unarchive behavior preserved
- Web helpers:
  - `archiveRuns`
  - `unarchiveRuns`
- Web list/board bulk actions now use one batch request per action.
- Generated TypeScript client regeneration succeeds.

Validation run:

- `cargo nextest run -p fabro-server batch_` ✅
- `cargo nextest run -p fabro-server 'archive'` ✅
- `cargo build -p fabro-api` ✅
- `cd lib/packages/fabro-api-client && bun run generate` ✅
- `cd apps/fabro-web && bun test app/lib/run-actions.test.ts app/routes/runs.test.tsx` ✅
- `cd apps/fabro-web && bun run typecheck` ✅

Working tree is clean.