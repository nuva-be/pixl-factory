All verify steps pass. The release build completed successfully in the tmpfs target. 

## Summary

The previous verify failure was a **transient infrastructure issue**: the 20G overlay disk was 100% full when the release build for `fabro-cli` ran, causing `aws-lc-sys` and `ring` C compilation to fail with "No space left on device" while writing assembler temp files. The code itself was fine (the prior `simplify_opus` stage had succeeded).

**Remediation:**
- Deleted the 18G `target/` directory on the overlay filesystem
- Mounted a 60G `tmpfs` at `target/` (the host has 755G RAM, 645G available), giving builds ample space

**Verify steps run individually, all pass:**
- `git fetch origin main` + `git merge` (already up to date)
- `cargo +nightly-2026-04-14 fmt --all` (no changes needed)
- `cargo dev docs refresh` (no changes)
- `cargo +nightly-2026-04-14 fmt --check --all` ✓
- regex sweep ✓ (no forbidden symbols)
- `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings` ✓
- `cargo nextest run --workspace --profile ci` ✓ (6499 tests pass, 181 skipped)
- `cargo dev docs check` ✓ (docs up to date)
- `bun install --frozen-lockfile` ✓
- `apps/fabro-web` `bun run typecheck` + `bun run test` ✓ (508 tests pass)
- `lib/packages/fabro-api-client` `bun run typecheck` ✓
- `cargo dev build -- -p fabro-cli --release` ✓ (5m 09s)

No source files were modified — the fix was purely environmental (disk space).