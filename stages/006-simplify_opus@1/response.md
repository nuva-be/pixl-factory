All tests pass. Let me summarize.

## Summary

Reviewed the `fabro variable` CLI change with three parallel agents (reuse, quality, efficiency).

### Findings reviewed
- **Code reuse:** flagged exact-duplicate `format_age` between `secret/list.rs` and `variable/list.rs`; also noted broader patterns (table boilerplate, color_choice) pre-existing in 8+ places.
- **Code quality:** flagged unused `Client::update_variable`, the duplicated `format_age`, divergent JSON envelope keys (`{name}` vs `{key}`), and intentional empty-value-acceptance for stdin.
- **Efficiency:** no meaningful issues — each subcommand makes a single API call.

### Fixed
1. Extracted `format_age` to `lib/crates/fabro-cli/src/shared/utilities.rs` and updated both `secret/list.rs` and `variable/list.rs` to import via `crate::shared::format_age`. Removes 20 lines of byte-identical duplication.

### Deliberately skipped (intentional per plan)
- **Empty value handling for `variable set`:** the plan (R3) explicitly requires empty values be allowed for variables, unlike secrets.
- **`set` as upsert via `POST`:** the plan explicitly chose this to match `fabro secret set`'s mental model.
- **`update_variable` Rust client wrapper:** the plan listed all five wrappers (list/get/create/update/delete) for API surface parity. Used by the web client; kept the Rust wrapper for completeness.
- **JSON envelope `{"name": ...}` for `variable rm`:** specified by the plan and matches the API field (the secret `{"key": ...}` is the outlier).
- **Broader cli_table/color_choice duplication:** pre-existing across 8+ commands; out of scope.
- **Project-wide `use func` import style nit:** pre-existing in secret too; not introduced by this change.

### Verification
- `cargo check -p fabro-cli`: clean
- `cargo +nightly-2026-04-14 fmt --check -p fabro-cli`: clean
- `cargo +nightly-2026-04-14 clippy -p fabro-cli --all-targets -- -D warnings`: clean
- `cargo nextest run -p fabro-cli -E 'test(/variable/)'`: 20/20 passed
- `cargo nextest run -p fabro-cli -E 'test(/secret/)'`: 30/30 passed (no regression from `format_age` extraction)