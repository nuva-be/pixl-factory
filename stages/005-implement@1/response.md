All 286 `fabro-types` tests pass (14 new tests added).

## Summary

I implemented the foundational step of the Unified Agent Transcript Events plan — step 1 of the plan's explicit migration order: **"Add canonical types and event deserialization support."**

### What was delivered

**New canonical module `fabro-types::transcript`** (new file `lib/crates/fabro-types/src/transcript.rs`, re-exported from `fabro_types::*`):
- `ContentPart`, `ThinkingData`, `ToolCall`, `ToolResult`, `ImageData`, `AudioData`, `DocumentData` — moved from `fabro-llm` to `fabro-types`. `fabro-llm::types` now re-exports them so every existing import path still works.
- New canonical types: `TranscriptMessage`, `MessageKind` (`system | user | reasoning | agent`), `MessageSource` (`system_prompt | turn_input | followup | steer | pair | injected_system | injected_user | loop_detection | provider_reasoning | provider_answer`), `MessageId` (ulid), `PairMessageRef`, `TranscriptUsage`.

**Extended `fabro-types::run_event::agent`**:
- `AgentMessageProps` gains `message: Option<TranscriptMessage>`.
- `AgentToolStartedProps` gains `tool_call: Option<ToolCall>`, `turn_id: Option<TurnId>`, `parent_message_id: Option<MessageId>`.
- `AgentToolCompletedProps` gains `tool_result: Option<ToolResult>`, `turn_id: Option<TurnId>`.
- All new fields are `#[serde(default, skip_serializing_if = "Option::is_none")]` so old wire payloads keep round-tripping unchanged.

**Tests (14 new, all green):**
- `transcript::tests`: 8 serde round-trip tests covering `ContentPart` (text, thinking with signature/redaction, opaque OpenAI items), `ToolCall` provider_metadata preservation, `ToolResult` defaults, `TranscriptMessage` round trip, `TranscriptMessage` optional-field omission, `PairMessageRef`.
- `run_event::agent::tests`: 6 tests proving back-compat (legacy JSON without canonical fields still deserializes) and forward shape (canonical payloads round-trip and preserve `provider_metadata` / linkage).
- All 36 pre-existing `fabro-llm::types` tests still pass — proves the type move is wire-compatible.
- 1254 `fabro-workflow` + `fabro-store` tests still pass.
- 997 `fabro-server` + `fabro-agent` tests pass (the 3 unrelated failures — graph SVG rendering — also fail on the pre-change baseline; they need a Graphviz subprocess that isn't available in this sandbox).

**Quality gates:**
- `cargo check --workspace` clean
- `cargo +nightly-2026-04-14 clippy --workspace --all-targets -- -D warnings` clean
- `cargo +nightly-2026-04-14 fmt --check` clean

### What is intentionally not done in this change

Tasks 2–8 of the plan (emitting canonical messages from `fabro-agent::Session`, plumbing `turn_id`/`parent_message_id` through tool execution, dual persistence in workflow + server session paths, projection from events, pair-message replay semantics, redaction policy, web/server consumer migration, OpenAPI changes) require touching session.rs (~3000 lines), the API path, server session handlers, web consumers, and the OpenAPI spec. The plan explicitly says: *"Keep narrow `text`, `model`, `billing`, and `tool_call_count` fields until web/server/client consumers are migrated"* and stages the rollout — so leaving the producers and consumers unchanged (still emitting / reading the narrow fields, with `message: None` etc.) is the correct intermediate state. The wire is now ready for emitters to start populating `message` / `tool_call` / `tool_result` / `turn_id` / `parent_message_id` incrementally without breaking existing payloads or consumers.