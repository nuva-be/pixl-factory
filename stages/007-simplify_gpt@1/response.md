Reviewed the feature diff against `origin/main` for reuse, quality, and efficiency.

Result: no additional cleanup changes were warranted; working tree remains clean.

Validation passed:

- `cargo nextest run -p fabro-server ...`
  - 6 focused lifecycle tests passed
- `cd apps/fabro-web && bun test app/lib/run-actions.test.ts app/routes/runs.test.tsx`
  - 27 frontend tests passed

I also confirmed `git status --short` is clean after validation.