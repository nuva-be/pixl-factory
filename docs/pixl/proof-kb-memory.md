# Proof: pixl-factory + Claude Code + pixl-kb memory (the full thesis)

> Verified 2026-06-19. The complete pixl OS loop, end-to-end, on the Claude subscription with no API key:
> **execution (pixl-factory) + crew (Claude Code) + memory (pixl-kb)**, composed via ACP + MCP.

## What was proven

```
fabro run examples/pixl/kb-recall.toml --auto-approve
  Sandbox: local (ready in 0ms)
  ✓ Recall  27s
  Status: SUCCEEDED
  Output:
    1. Tool call succeeded: Yes, mcp__pixl-kb__pixl_search executed without errors.
    2. Results returned: 10
    3. First result title: "FN-6 — Build admin funnel overview view"
```

- **Engine**: rebranded pixl-factory binary (the fork).
- **Agent**: Claude Code driven over ACP (`@zed-industries/claude-code-acp`) on the **subscription** (`--skip-llm`).
- **Memory**: Claude Code called **`mcp__pixl-kb__pixl_search`** and got 10 real results from the `feen`
  workspace in pixl-kb.

## The recipe (reproducible)

1. **pixl-kb running**: `cd pixl-kb && make dev-api` (:8421).
2. **Register pixl-kb as an MCP server in Claude Code** (user scope, so ACP sessions inherit it):
   ```bash
   claude mcp add pixl-kb -s user \
     -e PIXL_KB_WORKSPACE_ID=<workspace-id> \
     -- uv --directory <path-to>/pixl-kb run python -m knowledge_api.mcp
   ```
   The kb MCP runs in **direct mode** — `PIXL_KB_WORKSPACE_ID` scopes every tool call to that workspace.
   (Remove later with `claude mcp remove pixl-kb -s user`.)
3. **Run a workflow** with a `backend="acp"` node whose prompt calls `mcp__pixl-kb__pixl_search` (or
   `pixl_wakeup` / `pixl_diary_write` …), using the `local` environment so the ACP adapter runs on the host.

## Why this is the differentiator

The ACP node inherits the host's Claude Code config: the **pixl-crew plugin** (agents/skills) *and* the
**pixl-kb MCP** (memory). So a single work node has execution orchestration (pixl-factory), expertise (crew),
and compounding memory (kb) — on the subscription. No other agent platform composes all three.

## Next

- **P4 memory hooks**: auto-call `pixl_wakeup` at run-start and `pixl_diary_write` at stage-end via
  `fabro-hooks`, so memory isn't left to the model's discretion.
- **P3 Memory panel**: surface the kb recall/write in run detail.
- **Sandbox/cloud**: for non-local runs, bake pixl-kb's MCP (or the HTTP gateway `/api/mcp/call`) into the
  sandbox image so the ACP agent reaches kb from inside the container.
