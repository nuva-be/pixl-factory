The feature was already merged in PR #380 (`8ceb246b5`). The current branch's `implement` stage produced no diff — `git diff HEAD~4..HEAD` is empty — because the work was already on `main` and pulled in via the branch base.

There are no changes to review. The code is already in place and (per the preflight stages) compiles cleanly and lints with no warnings.

**Summary:** No diff exists between this branch's tip and its base. Nothing to fix. The batch archive/unarchive implementation lives at:
- `docs/public/api-reference/fabro-api.yaml` (OpenAPI contract)
- `lib/crates/fabro-server/src/server/handler/lifecycle.rs` (`batch_archive_runs` / `batch_unarchive_runs`)
- `apps/fabro-web/app/lib/run-actions.ts` (`archiveRuns` / `unarchiveRuns`)
- `apps/fabro-web/app/routes/runs.tsx` (bulk-action toolbar and board column menu)

These were merged via PR #380 prior to this branch being created, so the implement stage was a no-op.