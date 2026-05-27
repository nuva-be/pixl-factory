Implemented a broad React effects policy migration pass for `apps/fabro-web`.

What changed:
- Added an approved hook surface in `app/hooks/`, including:
  - generic primitives: `useMountEffect`, `useInterval`, `useTimeout`, `useDebouncedValue`, `useWindowEvent`, `useDocumentEvent`, `useDocumentTitle`, `useMediaQuery`, `useLocationHash`, `useResizeObserver`
  - purpose/domain hooks for xterm, Graphviz SVG annotation, Viz rendering, DOT language registration, run file deep links, refresh focus/spinner behavior, file tree model sync, live events, etc.
- Removed direct effects from many route/component files by replacing them with:
  - render-time derivation / keyed state patterns where applicable
  - event-path handling for run-detail lifecycle toasts
  - purpose-named integration hooks for DOM, timers, SSE, xterm, ResizeObserver, document title, Graphviz/Viz, URL hash, and file-tree model synchronization
- Migrated notable hotspots:
  - `routes/run-detail.tsx`: lifecycle mutation toasts now happen on the mutation trigger path instead of watching mutation results in an effect.
  - `routes/run-files.tsx`: media query, freshness ticking, refresh spinner cleanup/focus, hash tracking, deep-link focus/toast, and last-good-data handling moved behind named hooks.
  - `components/terminal-view.tsx`: xterm/WebSocket/ResizeObserver integration moved into `useTerminalSession`.
  - `routes/run-overview.tsx`: SVG DOM annotation/listener work moved into `useAnnotatedRunGraphSvg`.
  - `routes/runs.tsx`: derived columns reset effect replaced with render-time state derivation.
- Added short doc comments to new approved hooks naming the external integration and cleanup behavior.
- Updated the live-events test mock to include the new hook export.

Evidence / checks:
- Ran the policy search. Remaining non-hook/non-lib direct effects are now isolated to `install-app.tsx`:
  ```sh
  rg -n "\buseEffect\b|React\.useEffect|\buse(Layout|Insertion)?Effect\b" apps/fabro-web/app --glob '*.{ts,tsx}' --glob '!apps/fabro-web/app/hooks/**' --glob '!apps/fabro-web/app/lib/**' --glob '!**/*.test.tsx' --glob '!**/*.test.ts'
  ```
- `cd apps/fabro-web && bun run typecheck` ✅
- `cd apps/fabro-web && bun test --isolate` ✅ — 493 pass, 0 fail

Remaining uncertainty / next pass target:
- The full goal is not complete. `install-app.tsx` still contains direct `useEffect` calls inside documented install-specific hooks (`useInstallController`, `useInstallRootRedirect`). The goal itself called this area out as needing a more careful SWR/query or install state-machine migration, so I left behavior intact rather than doing a risky mechanical rewrite.