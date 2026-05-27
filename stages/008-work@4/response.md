Implemented more React Effects Policy cleanup and verification.

What changed:
- Replaced remaining child-runs search-param hydration effect behavior with render-time `<Navigate replace />`.
- Added child-runs preference hydration coverage to `runs.preferences.test.tsx`, including first-render query args and URL hydration.
- Exported `CHILD_RUNS_LIST_PREFERENCES_STORAGE_KEY` for focused test coverage.
- Made install GitHub callback-error consumption Strict Mode-safe so the second development effect pass does not clear a consumed callback error.
- Wrapped the install callback-error test in `StrictMode` to prove the behavior.
- Removed direct `useEffect` usage from two test helper mocks so full effect search output is cleaner.
- Tidied JSX indentation around the new render-time `<Navigate>` usage.

Evidence/checks run:
- Production route/component direct-effect search excluding hooks/lib/tests: no matches.
- Full direct-effect search now reports only hook/lib integration sites.
- `cd apps/fabro-web && bun run typecheck` passed.
- `cd apps/fabro-web && bun test --isolate` passed: 495 tests, 0 failures.
- Focused tests passed for:
  - `app/routes/runs.preferences.test.tsx`
  - `app/install-app.test.tsx`
  - `app/components/toast.test.tsx`
  - `app/routes/settings-live-events.test.tsx`

Remaining uncertainty:
- The remaining direct effects are confined to purpose-named hooks/lib integrations such as timers, browser URL/history/storage, DOM/Pierre/Viz/xterm integrations, and SSE subscriptions. They look aligned with the policy, but the next audit should still make the final completion decision.