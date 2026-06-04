All verify steps pass:

- ✅ `cargo fmt --check --all`
- ✅ `cargo dev docs refresh` (no changes)
- ✅ Forbidden pattern grep in verify scope (no matches)
- ✅ `cargo clippy --workspace --all-targets -- -D warnings`
- ✅ `cargo nextest run --workspace` (6503 passed, 181 skipped)
- ✅ `cargo dev docs check`
- ✅ `bun install --frozen-lockfile`
- ✅ `apps/fabro-web`: typecheck + 508 tests pass
- ✅ `lib/packages/fabro-api-client`: typecheck
- ✅ `cargo dev build -- -p fabro-cli --release`

The previous verify failure was a transient 10-minute timeout on the entire verify chain (the release build alone takes 2:30). No actual code issues — nothing to fix.