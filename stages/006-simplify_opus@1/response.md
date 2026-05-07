Net 73-line reduction with quality improvements. Let me summarize the simplification work:

## Summary

I launched three parallel review agents (code reuse, quality, efficiency) against the diff for "Move GitHub token permissions to `[run.integrations.github.permissions]`" and aggregated their findings.

### Fixes applied

**1. Replaced stringly-typed `clone_sandbox_*_github_credentials(&str)` helpers with `SandboxProvider::is_clone_based()`** *(both reuse + quality reviewers flagged)*
- Deleted `clone_sandbox_requires_github_credentials` in `fabro-cli/src/commands/run/runner.rs` (newly introduced by this diff).
- Deleted `clone_sandbox_can_use_github_credentials` in `fabro-server/src/server.rs` (pre-existing duplicate).
- Both call sites now `parse::<SandboxProvider>()` and call `is_clone_based()` — same logic, but bypasses no-longer-needed string helpers and uses the typed enum that already exists in `fabro-sandbox`.
- Removed the corresponding unit tests (now redundant with `SandboxProvider`'s own tests).
- Added `fabro-sandbox` import to `runner.rs` (the dep was already in `Cargo.toml`).

**2. Eliminated hand-rolled `Combine for RunIntegrationsGithubLayer`** *(both reuse + quality reviewers flagged)*
- Added one-line `impl Combine for Option<HashMap<String, InterpString>>` in `combine.rs` next to the existing `Option<HashMap<String, toml::Value>>` impl.
- Switched `RunIntegrationsGithubLayer` to `#[derive(fabro_macros::Combine)]` like every sibling layer.
- Deleted the 13-line "do not derive" justification comment; kept a shorter doc-comment explaining the `Some({})`-as-clear semantics.
- Removed the now-unused `use super::combine::Combine` import.

**3. Replaced duplicate API DTOs with `with_replacement` entries** *(reuse reviewer flagged, per CLAUDE.md API-type-ownership policy)*
- Added `RunIntegrationsSettings` and `RunIntegrationsGithubSettings` to the `replacements` list in `fabro-api/build.rs`. The generated API client now reuses the canonical `fabro_types::settings::run::*` types instead of producing parallel DTOs.
- Deleted `fabro-api/tests/run_integrations_round_trip.rs` (3 tests, 64 lines) — JSON parity is now structural identity rather than a runtime assertion.

### Findings noted but not addressed

- **Extract `requires_github_credentials` and `process_env_var` into shared crates.** Multiple reviewers flagged 3 near-duplicate gate expressions and 4 copies of `process_env_var`. The gate-extraction touches semantically different sites (origin-presence, dry-run, soft vs. hard) beyond this diff's scope; the `process_env_var` duplication is a pre-existing workspace pattern used in ~10 places.
- **`ClearableMap<V>` newtype.** Quality reviewer suggested moving the empty-as-clear semantics into a peer of `ReplaceMap`/`StickyMap`/`MergeMap`. The existing `Option<HashMap<...>>: Combine` solution is simpler with the macro already in place.
- **`prepare_manifest` parsing through `SettingsLayer`.** Efficiency reviewer flagged this as bounded acceptable cost (deliberate to enable `deny_unknown_fields` validation). No fix needed.

### Verification

- `cargo check --workspace` clean.
- `cargo nextest run -p fabro-config -p fabro-api -p fabro-cli -p fabro-server`: 1620 tests pass.
- `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings` clean.
- `cargo +nightly-2026-04-14 fmt --check --all` clean.

Net change: **+37 / −110 lines**.