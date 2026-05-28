Summary of changes to fix the verify failures:

1. **`lib/crates/fabro-cli/src/commands/run/create.rs`**: Added the missing `environment_defaults` field to `ManifestBuildInput`, populated with the built-in seeded environment catalog.

2. **`lib/crates/fabro-environment/src/{lib,store}.rs`**: Added a new public `seeded_catalog_layer()` helper that returns the built-in default/local/docker/daytona catalog as a `MergeMap<EnvironmentLayer>`, so CLI/manifest paths can validate environment ids without a live server `EnvironmentStore`.

3. **CLI manifest call sites** (`graph.rs`, `preflight.rs`, `run/create.rs`, `validate.rs`, `user_config.rs`): Inject the seeded catalog when calling `build_run_manifest`/`RunSettingsBuilder`.

4. **`lib/crates/fabro-config/src/builders.rs`**: Added public `RunSettingsBuilder::{load_default,load_from,from_toml}_with_catalog` variants so the CLI can merge a default catalog before resolution.

5. **`lib/crates/fabro-server/src/{manifest_validation,run_manifest,run_tool_manifest}.rs`**: Added `validate_manifest_with_environment_defaults`, made the base `validate_manifest` use the seeded catalog, removed the now-unused `prepare_manifest` wrapper, and made tool-manifest use the seeded catalog.

6. **`lib/crates/fabro-workflow/{Cargo.toml,src/operations/{create,start}.rs}`**: Added `fabro-environment` dev-dep and updated test settings builders to inject the seeded catalog.

7. **Tests and fixtures**: Updated `workflow_settings_round_trip.rs`, `config.rs`, `dump.rs`, `run.rs`, `support.rs` (CLI tests) and the variables API test to stop using project-config-defined `[environments.<id>]` blocks (now invalid) and seed the resolver with a catalog where needed.

8. **`.fabro/project.toml`**: Removed `[environments.fabro-dev]` block since project configs can no longer define environments per the new architecture.