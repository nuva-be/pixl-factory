All done. Here's the summary:

## Summary

I ran three parallel review agents (reuse, quality, efficiency) on the Automations Backend API diff, then aggregated and fixed the highest-impact findings.

### Fixed

**Correctness / dead code**
- `AutomationStoreError::Validation(InvalidWorkflowSelector)` was incorrectly returned for TOML *serialize* failures. Added a real `Serialize` variant mapped to 500.
- Removed the unreachable `AutomationStoreError::MissingRevision` variant (the handler enforces `If-Match` via `parse_if_match`; the store was never constructed without one).
- Removed `AutomationRunMaterializeError::WorkflowNotFound` plus the substring-matching `classify_manifest_error` (all variants collapsed to 422, classification was fragile).
- Dropped the unused `automation_id` field on `AutomationRunMaterializeInput` (was only used by the removed substring classifier).

**Anti-patterns**
- Removed the `AutomationRevision::from_bytes(b"")` placeholder dance. Persist now computes the revision from the canonical TOML bytes before assembling the `Automation`, so `revision` is never a known-lie.
- Dropped the misleading `impl FromStr for AutomationRevision` whose `Err` type was unreachable. Replaced with `AutomationRevision::from_raw(...)` which doesn't lie about validation.
- Replaced the bespoke `atomic_write` (`AtomicU64` counter + manual temp filename + `OpenOptions::create_new` + `rename`) with `tempfile::NamedTempFile::new_in(...).persist(...)` inside `spawn_blocking`, matching the idiom already used in `fabro-config/src/daemon.rs` and `fabro-vault`.
- Introduced a borrowed `PersistedAutomationRef<'a>` for serialization to avoid cloning the entire automation on every write.

**Reuse wins**
- `GitAutomationRunMaterializer::authenticated_clone_url` now calls `fabro_github::resolve_authenticated_url` instead of re-implementing `resolve_clone_credentials` + `embed_token_in_url`.
- Replaced the bespoke `redact_command_output` / `redact_url_token` (which split on whitespace and dropped newlines) with `fabro_sandbox::redact::redact_auth_url`, matching `run_manifest::check_git_remote_ref`. The auth URL is now threaded into `run_git` so credentials in stderr are stripped without destroying multi-line structure.

**Cleanup / hygiene**
- Removed unused `chrono` dependency from `fabro-automation/Cargo.toml`.
- Changed `pub mod error/id/model` → `mod` so the crate has a single public surface (the `pub use` re-exports).
- Dropped redundant sort in `list_automations` (BTreeMap iteration is already id-ordered).
- Materialized checkout dirs (`scratch/automations/<run_id>`) are now removed after the manifest is built — both on success and failure — instead of leaking full clones.

### Skipped (with reason)

- "`Duration::from_mins` is nightly-only" — false; it's stable since 1.83, and clippy actually *requires* it under `duration_suboptimal_units`.
- Replacing `Raw*` DTO sprawl with envelope/per-variant `deny_unknown_fields` — works as-is, refactor too invasive for cleanup pass.
- Switching `AutomationListResponse` to shared `PaginationMeta` — requires OpenAPI yaml + TypeScript regen.
- `Arc<Automation>` in the store, per-automation run index, etc. — premature for current scale.
- Typed-id `AutomationRef` in `fabro-types` — would require a crate dependency flip; out of scope.

### Verification

- `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo +nightly-2026-04-14 fmt --check --all`: clean.
- `cargo nextest run -p fabro-automation`: 23/23 passing.
- `cargo nextest run -p fabro-api`: 158/158 passing.
- `cargo nextest run -p fabro-server --features test-support --test it -E 'test(/automation/)'`: 6/6 passing.
- `cargo nextest run -p fabro-server`: 643/647 passing. The 4 failures (`build_manifest_from_checkout_resolves_workflow_path`, `get_graph_returns_svg`, `render_graph_from_manifest_returns_svg`, `render_graph_from_manifest_accepts_fabro_dotted_attributes`) are **pre-existing on `afdd4900f`** — verified by `git stash` + retest; they appear to be host-environment / external-binary issues (rendering subprocess, manifest workflow resolution) unrelated to this cleanup.