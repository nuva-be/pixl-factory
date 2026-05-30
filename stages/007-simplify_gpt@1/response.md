Review/cleanup complete.

What I found:
- Current branch diff vs `origin/main` only contains one small change in `apps/fabro-web/app/routes/automations.tsx`.
- I launched the requested three parallel review agents for reuse, quality, and efficiency review.
- No actionable cleanup issues were identified.
- The current `runDisabled = busy || running || !automation.apiEnabled` simplification is reasonable and removes the redundant `running || runDisabled` disabled expression.

Validation run:
- `cd apps/fabro-web && bun run typecheck` ✅
- Searched touched areas for leftover top-level automation enabled references:
  - No removed master-gate references found.
  - One expected trigger-related hit remains: `automation.enabled_api_trigger()` in the server handler.