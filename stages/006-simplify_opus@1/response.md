## Summary

Reviewed the Environment REST CRUD API diff (2,574 lines) with three parallel agents covering code reuse, code quality, and efficiency.

### Fixed

**Collapsed API DTO duplication in `lib/crates/fabro-server/src/server/handler/environments.rs`** (-27 net lines):
- Removed the redundant `ApiEnvironmentSettings` middleman struct (an exact field-by-field clone of `CreateEnvironmentRequest`/`ReplaceEnvironmentRequest`).
- Removed both `into_settings`/`into_draft` trampoline impls that constructed `ApiEnvironmentSettings` only to shell out to a shared converter.
- `CreateEnvironmentRequest::into_draft` and `ReplaceEnvironmentRequest::into_settings` now build `EnvironmentSettings` directly.
- Replaced `ApiDockerfileSource::Path { _path: String }` with `_path: IgnoredAny`, removing the per-request `String` allocation for a payload that's parsed and discarded.

### Verified

- `cargo +nightly clippy --workspace --all-targets -- -D warnings` ✅
- 13/13 `api::environments` tests pass
- 4/4 `openapi_conformance` tests pass
- 170/170 `fabro-api` tests pass (including round-trip)
- 16/16 `fabro-environment` tests pass

### Reviewed but not fixed (with rationale)

- **Shared etag/parse helpers across automations.rs and environments.rs** (High priority from reuse review): The five helpers (`parse_path_id`, `parse_required_if_match`, `unquote_etag`, `*_with_etag_response`, `From<*StoreError> for ApiError`) are byte-twins with the automations versions. Extracting them requires changes to both modules and adds trait machinery; better tackled when a third resource lands.
- **Catalog deep clone in `resolve_manifest_run_settings_with_catalog`** (Medium efficiency): Would require restructuring `SettingsLayer.environments` from owned `MergeMap` to `Arc<MergeMap>` across fabro-config. Environment writes are admin-only on small catalogs; deferred.
- **`Environment::from_settings` extra resolve round-trip** (Medium efficiency): The added `resolve_environment(&persisted)` call is intentional — it's the validation step that powers the new `create_invalid_settings_is_rejected` test. Correct as-is.
- **List pagination** (Medium efficiency): Out of scope; would require an OpenAPI spec change.
- **`EnvironmentListResponse`/`EnvironmentListMeta` generalization** (Medium reuse): Same pattern as `AutomationListResponse`; cross-cutting refactor worth doing later.
- **OpenAPI YAML repeats every 4xx/5xx response body** (Low quality): Pre-existing pattern across automations and other paths; systemic, not introduced by this PR.