No leftover auth-mode references. Everything passes:

- **Root cause**: The local git clone had a narrow fetch refspec (`feat/settings-environments-crud` only), so `git fetch origin main` only updated `FETCH_HEAD` without creating `refs/remotes/origin/main`, making `git merge origin/main` fail.
- **Fix**: Added a `fetch = +refs/heads/main:refs/remotes/origin/main` line to `.git/config`'s `[remote "origin"]` so `origin/main` becomes a real remote-tracking branch.

After that fix, the full verify pipeline succeeds end-to-end:
- Merge from `origin/main` (clean ort-strategy merge of Cargo.lock and two server files)
- `cargo fmt --check --all` clean
- `cargo dev docs refresh`/`check` clean
- `cargo clippy --workspace --all-targets -D warnings` clean
- `cargo nextest run --workspace --profile ci`: 6494 passed, 181 skipped
- `bun install --frozen-lockfile` ok
- `apps/fabro-web` `bun run typecheck` and `bun run test`: 508 pass
- `lib/packages/fabro-api-client` `bun run typecheck` clean
- `cargo dev build -- -p fabro-cli --release` succeeds
- Auth-mode grep returns no leftovers