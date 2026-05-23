Reviewed the compaction changes vs `origin/main` and launched the three requested parallel review agents.

Fixed one correctness/quality issue found during review:

- `estimate_active_context_usage()` local fallback was dividing the system prompt and turns separately, which could differ from the previous whole-history `chars / 4` behavior due to integer rounding.
- Updated it to sum system prompt chars + turn chars first, then divide once.
- Reused the same local char/token helper for fallback, post-baseline delta, and summary token estimate.
- Added a regression test proving whole-history rounding is preserved.

Verification run:

- `cargo nextest run -p fabro-agent compaction` ✅
- `cargo nextest run -p fabro-agent history` ✅
- `cargo +nightly-2026-04-14 fmt --check --all` ✅
- `cargo nextest run -p fabro-agent` ✅ 448 passed
- `cargo +nightly-2026-04-14 clippy -q --package fabro-agent --tests --no-deps -- -D warnings` ✅

Note: a broader `cargo +nightly-2026-04-14 clippy -q -p fabro-agent --all-targets -- -D warnings` hit an unrelated dependency lint in `fabro-sandbox::SandboxSpec` (`large_enum_variant`), so I reran clippy scoped to `fabro-agent` without dependencies, which passed.