Implemented review cleanup after running the three parallel review agents.

Fixed:
- **Local bus replay correctness**
  - `Start` subscriptions now fail with `InvalidCursor` if the local stream has already trimmed messages, rather than silently replaying a truncated stream.
  - Active `Start` subscribers that joined before first publish also fail if messages were trimmed before they could establish a cursor.
  - `After(id)` subscriptions for unknown runs no longer create empty run stream entries.
  - Added tests for these cursor/cleanup cases.

- **Worker-side delivery-id dedupe**
  - Replaced the single `last_applied_id` duplicate check with a bounded recent-id dedupe set.
  - Uses FIFO + `HashSet`, capped at `2048` delivery ids.
  - Still uses the last applied id for reconnect `?after=...`.
  - Prevents repeated non-adjacent duplicate delivery ids from being applied twice without unbounded memory growth.

- **Formatting**
  - `worker_control.rs` had rustfmt-only import formatting changes.

Already clean / previously addressed in current tree:
- Worker control route already checks invalid cursor before WebSocket upgrade and returns HTTP `410 Gone`.
- Invalid-cursor close reason and ping/pong constants were already shared through `fabro-interview`.
- Terminal-run check already uses `is_terminal()`.

Validation run:
- `cargo +nightly-2026-04-14 fmt --check --all` ✅
- `cargo nextest run -p fabro-server worker_control` ✅
- `cargo nextest run -p fabro-cli runner` ✅
- `cargo +nightly-2026-04-14 clippy -q -p fabro-server -p fabro-cli --all-targets -- -D warnings` ✅