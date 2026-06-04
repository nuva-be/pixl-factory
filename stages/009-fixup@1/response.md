Summary:

**Failure cause:** Transient infrastructure issue. The verify script chains the full debug pipeline (`cargo nextest run --workspace --profile ci`, which produced ~17 GB of artifacts in `target/debug/`) immediately followed by a release build (`cargo dev build -- -p fabro-cli --release`). The accumulated debug + in-progress release artifacts exceeded the 20 GB overlay disk, causing `aws-lc-sys` C compilation to abort with "No space left on device" / SIGABRT.

**Resolution:** No code changes were required. All prior verify steps (fmt, clippy, nextest, docs check, bun typecheck/test) had already succeeded — the failure was strictly the final release build step running out of disk. After freeing space by removing `target/debug`, the release build completes cleanly:

- `cargo dev build -- -p fabro-cli --release` → `Finished release profile [optimized] target(s) in 4m 15s`
- Produced `target/release/fabro` (≈111 MB)
- Disk now at 28% used, leaving plenty of headroom

The next verify retry should succeed on a fresh sandbox where debug and release artifacts don't have to coexist mid-run, or with the same workaround if the orchestrator clears debug before the final release build.