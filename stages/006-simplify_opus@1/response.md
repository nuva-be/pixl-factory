All fixes applied. Summary of cleanup:

## What was fixed

**TypeScript — `ask-fabro-runtime.ts`:**
- Removed the unused `textParts` field on `TurnAccumulator` (dead state, all 3 reviewers flagged it).
- Removed the unused `attachSessionEvents` import and `attachSessionEventsImpl` test seam.
- Extracted a `wakeWaiter()` helper to dedupe the 3× `if (resolveWaiter) { ... }` block.
- Replaced manual `let streamError` + `.then(_, err)` capture with a `try/finally` IIFE so `await streamPromise` propagates errors naturally.
- Added a `yielded` flag so the post-loop `yield snapshot(acc)` only fires for empty turns, avoiding a redundant re-yield of the last in-loop snapshot on every successful turn.
- Simplified `lastUserText` to a `for...of` loop over a typed content part shape, dropping the `.map().filter().join()` chain and the multiple `unknown` casts.

**TypeScript — `run-detail.tsx`:**
- Typed `ASK_FABRO_UNAVAILABLE_TOOLTIPS` as `Record<AskFabroUnavailableReasonEnum, string>` using the api-client enum, so adding a new enum variant fails compilation until the map is updated.

**Rust — `fabro-workflow/handler/llm/api.rs`:**
- Replaced `register_fabro_run_tools_subset(..., only: &[&str])` with the "empty means all" footgun by introducing `register_named_fabro_run_tools(..., names: &[&str])` (registers only listed names) alongside the existing `register_fabro_run_tools` (registers all). Test renamed and a new "unknown name is ignored" test added.

**Rust — `fabro-server`:**
- Simplified `AppState::self_server_target` by parsing `Bind::to_target()` via `ServerTarget`'s `FromStr`, dropping the manual `tcp_port().is_some()` branch.
- Replaced `anyhow::anyhow!("{err:?}")` (which Debug-formats `ApiError`) with the same `"failed to sign worker token"` message already used in `worker_command`.

**Cosmetic:** Added trailing newlines to 4 touched files (ask-fabro-sidebar.tsx, run-detail.tsx, ask-fabro.tsx, fabro-server/Cargo.toml; ask-fabro-runtime.ts was rewritten with one).

## What was intentionally skipped

- **OpenAPI schema fix for `EventEnvelope` nesting** — would ripple through generated clients across the workspace; the cast comment correctly documents the underlying issue.
- **Generated event-name constants** in fabro-api-client — requires codegen changes outside this PR's surface.
- **Caching `ServerTarget` on `AppState` at startup** — real efficiency win but adds startup-ordering complexity (the server writes the daemon record itself).
- **`worker_command` ↔ `self_server_target` dedup** — pre-existing duplication, beyond this PR.
- **Restructuring `FabroRunToolServices` to use `Option<PathBuf>`** — touches the workflow's struct contract used by many callers.
- **Per-session `reqwest::Client` reuse** — low severity; one sidebar per run in practice.
- **Splitting `AskFabroAdapterOptions` into options + DI bag** — minor style preference.

## Verification

- `cargo build --workspace` ✅
- `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings` ✅
- `cargo nextest run -p fabro-workflow` ✅ (1116 passed)
- `cargo nextest run -p fabro-server --features test-support` ✅ (721 passed)
- `bun run typecheck` ✅
- `bun test app/lib/ask-fabro-runtime.test.ts` ✅ (5 passed)
- `bun test` — same 7 pre-existing failures as the pre-cleanup baseline (verified via `git stash`); no regressions introduced.

Cleanup committed as `ffcb411a1`.