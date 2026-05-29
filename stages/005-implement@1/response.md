Implemented the frontend-only “create automation from run” prefill flow.

Changes made:
- Added `automationFormValuesFromRun(run, settings)` in `automation-form.tsx`.
  - Prefills name, slug, repo, branch, workflow, and trigger defaults.
  - Keeps repo parsing intentionally GitHub-only/narrow.
- Wired `/automations/new?from_run=<run_id>` in `automations-new.tsx`.
  - Uses `useSearchParams`, `useRun`, and `useRunSettings`.
  - Splits route wrapper from keyed form child.
  - Preserves blank form behavior.
  - Shows loading while source run loads.
  - Falls back to editable empty form with a non-blocking error if source run can’t load.
- Added run detail actions:
  - Ordinary runs: `Create automation from run`
  - Automation-linked runs: `View automation`
- Added route-level prefill tests in `automations-new.test.tsx`.
- Extended `run-detail.test.ts` for create/view automation navigation.

Verification:
- `cd apps/fabro-web && bun test app/routes/automations-new.test.tsx app/routes/run-detail.test.ts` ✅
- `cd apps/fabro-web && bun run typecheck` ✅
- `cd apps/fabro-web && bun test` ✅ — 508 tests passed

No backend, OpenAPI, generated client, or scheduler files were changed.