Let me do a quick sanity check on the plan's "final sanity checks":

1. ✅ Memory events never contain file contents — verified by test `agent_memory_loaded_payload_excludes_file_contents` and structural design.
2. ✅ Skills discovered and memory loaded emitted even for empty lists — verified by tests `initialize_emits_memory_loaded_event_with_empty_files_when_no_memory` and `initialize_emits_skills_discovered_event_when_no_skills`.
3. ✅ Skill activation is persisted rather than filtered as streaming noise — `SkillActivated` is not in `is_streaming_noise()` and has conversion + name mapping.
4. ✅ `agent.mcp.ready` backwards-compatible — verified by `agent_mcp_ready_deserializes_legacy_payload_without_tools` test and `#[serde(default, skip_serializing_if = "Vec::is_empty")]`.
5. ✅ Docs match the serialized event names and payload shapes.

Summary of changes:

**Task 1 (fabro-types):** Added `AgentMemoryLoadedProps`, `AgentMemoryFileProps`, `AgentSkillsDiscoveredProps`, `AgentSkillSummary`, `AgentSkillActivationSource`, `AgentSkillActivatedProps`, `AgentMcpToolSummary`. Extended `AgentMcpReadyProps` with a backwards-compatible `tools` field. Added 3 new `EventBody` variants (`AgentMemoryLoaded`, `AgentSkillsDiscovered`, `AgentSkillActivated`) plus the legacy whitelist entries. Added 5 serde tests.

**Task 2 (fabro-agent / fabro-workflow):** Added internal `AgentEvent::MemoryLoaded`, `SkillsDiscovered`, `SkillActivated` variants plus support structs (`MemoryFileSummary`, `SkillSummary`, `McpToolSummary`, `SkillActivationSource`). Extended `AgentEvent::McpServerReady` with `tools`. Added tracing for the new variants. Updated workflow `convert.rs` and `names.rs`. Added 4 conversion tests.

**Task 3 (memory):** Changed `discover_memory` to return `Vec<MemoryDocument>` with `{path, content, byte_count, loaded_bytes, truncated}`. `Session::initialize` emits `MemoryLoaded` immediately after discovery, always (even for empty results). The prompt assembly receives content-only strings. Memory test added for truncation metadata.

**Task 4 (skills):** `Session::initialize` emits `SkillsDiscovered` (always). Slash expansion now emits `SkillActivated { Slash }` instead of `SkillExpanded`. `make_use_skill_tool` emits `SkillActivated { Tool }` only on successful lookup. Failed lookups do not emit. Added 4 session tests.

**Task 5 (MCP):** Added `McpConnectionManager::tool_summaries_for_server` that filters/sorts by qualified name. `Session::initialize` includes the summaries in `McpServerReady`. Existing MCP e2e test now asserts the names-only summary content. Added an mcp unit test.

**Task 6 (docs):** Updated `docs/internal/events.md` with the enriched `agent.mcp.ready` payload, three new sections for `agent.memory.loaded`, `agent.skills.discovered`, `agent.skill.activated`, and a note about the retired `agent.skill.expanded` notification.

**Task 7:** All 1841 tests in `fabro-agent`, `fabro-workflow`, `fabro-types`, `fabro-mcp` pass; workspace clippy clean under nightly-2026-04-14; formatting checked.