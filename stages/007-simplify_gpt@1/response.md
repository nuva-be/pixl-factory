Review complete.

- Ran diff review against `origin/main...HEAD`.
- Launched all three parallel review agents:
  - Code reuse
  - Code quality
  - Efficiency
- Aggregated findings.

Outcome:
- No code changes were needed after review.
- The main findings were caveats:
  - Reusing `fabro_automation::AutomationTarget` intentionally couples the public API shape to the domain model now that `ref` is required.
  - Unknown trigger discriminator → 422 is a handler behavior concern; no automation HTTP handlers are implemented in this diff.
  - Persisted automation TOML shape changed from prior optional/`cron` fields as part of aligning the domain type with the public API contract.
- No duplicate helper logic, generated client issues, or efficiency problems were found.

Verification run:

```bash
cargo build -p fabro-api
cargo nextest run -p fabro-api
cd lib/packages/fabro-api-client && bun run generate
git diff -- apps/fabro-web lib/crates/fabro-cli
```

Results:
- `cargo build -p fabro-api`: passed
- `cargo nextest run -p fabro-api`: passed, 166/166
- TypeScript client regeneration completed and produced no additional diff
- No changes in `apps/fabro-web` or `lib/crates/fabro-cli`
- Working tree is clean relative to the current branch.