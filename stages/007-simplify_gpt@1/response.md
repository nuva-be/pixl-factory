Implemented one cleanup from the simplify review:

- Optimized routing-field raw text detection in `structured_output.rs` by replacing per-check `format!("\"{field}\"")` allocations with a static quoted-field list.

No other actionable reuse/quality/efficiency issues were found during the review pass.

Validation run:

- `cargo nextest run -p fabro-workflow structured_output handler::agent handler::prompt handler::llm::api handler::llm::acp` ✅
- `cargo nextest run -p fabro-workflow` ✅ 1160 passed
- `cargo +nightly-2026-04-14 fmt --check --all` ✅
- `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings` ✅

Changed file:

- `lib/crates/fabro-workflow/src/handler/structured_output.rs`