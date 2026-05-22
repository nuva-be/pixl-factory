All changes look correct, formatted, and tested. 

## Summary of cleanup applied

Three review agents found a mix of issues; I fixed the ones clearly within this diff's responsibility and skipped the larger refactors that would have expanded scope beyond the feature.

**Fixed:**

1. **Trailing newline regression** — restored final `\n` on three files that lost it during the implement phase: `docs/public/administration/server-configuration.mdx`, `docs/public/execution/run-configuration.mdx`, `lib/packages/fabro-api-client/src/models/run-checkpoint-settings.ts`.

2. **Field-name inconsistency** — renamed `GitState.checkpoint_skip_hooks` → `checkpoint_skip_git_hooks` in `sandbox_git.rs` and its two readers (`handler/parallel.rs`, `pipeline/execute.rs`). The feature is named `skip_git_hooks` everywhere else (settings, OpenAPI, TS, MDX, accessor); the local field was the only outlier dropping `_git_`.

3. **Shell quoting in `parallel_branch_commit_cmd`** — the new helper interpolated `author_name`, `author_email`, and `message` directly inside literal single-quoted strings, violating the CLAUDE.md "always use `shell_quote()`" rule. Rewrote it to use `fabro_sandbox::shell_quote` for all three values, matching `sandbox_git.rs::git_checkpoint`'s style. Existing substring-based tests for `--no-verify` and `commit --allow-empty` continue to pass.

4. **Rustfmt alignment** — `cargo fmt` re-aligned struct-field colons in `GitState` after the rename.

**Verification:** `cargo check --workspace`, `cargo +nightly-2026-04-14 fmt --check --all`, `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings` all clean. `cargo nextest run -p fabro-workflow -p fabro-config -p fabro-api` passed all 1462 tests; the targeted `fabro-cli` attach/inspect tests passed 34/34.

**Skipped (out of scope):**

- Extracting a unified commit-command builder between `git_checkpoint` (uses `-F <file>`) and `parallel_branch_commit_cmd` (uses `-m <msg>`). Real DRY, but the two differ on enough axes (message source, exclude pathspecs, metadata trailers) that it's a separate refactor.
- Replacing the 9-positional-arg `git_checkpoint` signature with an options struct. Predates this diff (already has `#[allow(clippy::too_many_arguments)]`); changing it would touch many call sites unrelated to the feature.
- Switching `self.skip_git_hooks.or(other.skip_git_hooks)` to `.combine(...)` in `combine.rs`. The surrounding `impl_combine_or_option!` macro is defined as `self.or(other)`, so the two are literally identical — pure style and the file's hand-rolled impls already use `.or()`-shaped logic.
- Hoisting `skip_git_hooks` outside the `for setup in branch_setups` loop. Efficiency agent flagged this then dismissed it as zero-cost.