All verify steps pass cleanly:

- `cargo fmt --check` ✓
- Banned-token sweep ✓
- `cargo clippy --workspace --all-targets -D warnings` ✓
- `cargo nextest run --workspace` → 6494/6494 passed ✓
- `cargo dev docs check` ✓
- `bun install --frozen-lockfile` ✓
- `apps/fabro-web` typecheck ✓
- `apps/fabro-web` tests → 508/508 passed ✓
- `lib/packages/fabro-api-client` typecheck ✓
- `cargo dev build -- -p fabro-cli --release` ✓

The previous verify failure was classified as `transient_infra` with signature `failed to spawn script`, meaning the shell driver itself failed to start the verify script — there was no code defect to fix. Re-running the full verify pipeline now succeeds end-to-end with no changes required.