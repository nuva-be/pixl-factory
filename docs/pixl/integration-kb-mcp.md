# pixl-kb ↔ pixl-factory: MCP integration design (task P0)

> Grounded in the code/docs of both sides. This is the wiring design; the live proof needs pixl-kb running
> + a model key (see "What's blocked").

## The two sides

- **pixl-factory (Fabro fork)** configures MCP servers in the run TOML:
  ```toml
  [mcp_servers.pixl-kb]
  type    = "stdio"
  command = ["...", "..."]
  [mcp_servers.pixl-kb.env]
  PIXL_KB_WORKSPACE_ID = "<workspace>"
  ```
  Agents auto-discover and call the tools (`mcp__pixl-kb__pixl_search`, etc.). Source: `docs/public/agents/mcp.mdx`.
- **pixl-kb** ships a **stdio MCP server** with 18 tools, launched on the host as:
  ```bash
  PIXL_KB_WORKSPACE_ID=<id> uv run python -m knowledge_api.mcp     # from the pixl-kb repo
  ```
  Tools we care about: `pixl_search`, `pixl_iterative_retrieve`, `pixl_corrective_retrieve`,
  `pixl_assess_sufficiency`, `pixl_diary_read/write`, `pixl_save_memory`, `pixl_wakeup`,
  `pixl_kg_query`, `pixl_kg_timeline`, `pixl_add_document`. Also a FastAPI HTTP API on :8421.

## The catch (why it's not a one-line config)

Fabro runs each agent **inside a sandbox** (Docker/Daytona). A `[mcp_servers]` stdio `command` executes **in the
sandbox**, where `uv` + the pixl-kb source do not exist. So the naive stdio config fails in sandboxed runs.
Three viable approaches:

| Approach | How | Trade-off |
|---|---|---|
| **A. stdio in sandbox image** | bake pixl-kb (or a thin MCP shim) into the sandbox image; stdio command runs locally in the container; the shim talks to kb's API at the host | clean MCP semantics; needs a custom sandbox image + host-reachable kb |
| **B. HTTP-MCP bridge** | run a tiny MCP-over-HTTP server in front of pixl-kb's :8421 API; configure Fabro with an `http` MCP transport | one bridge process; depends on Fabro MCP supporting `type="http"` (confirm in `fabro-mcp/http_transport`) |
| **C. local-sandbox only (dev)** | `sandbox.provider = local`; stdio `uv run python -m knowledge_api.mcp` runs on the host where kb lives | works today for the owner's single-machine dev loop; not for client/cloud runs |

**Recommendation:** start with **C** for the P0 proof (fastest — no image work), then build **A** (kb shim in the
sandbox image) for real sandboxed/cloud runs. `B` only if we want kb reachable by non-pixl MCP clients too.

## P0 proof config (approach C, local sandbox)

```toml
# in the run's .toml (or workflow defaults)
[sandbox]
provider = "local"

[mcp_servers.pixl-kb]
type    = "stdio"
command = ["uv", "--directory", "/Users/hamzamounir/code/nuva/pixl/pixl-kb", "run", "python", "-m", "knowledge_api.mcp"]
[mcp_servers.pixl-kb.env]
PIXL_KB_WORKSPACE_ID = "<a real workspace id>"
```
Then a one-node workflow whose agent is told to call `pixl_search`/`pixl_wakeup`, and we confirm the
`mcp__pixl-kb__*` tool calls appear in the run events.

## Auto-memory (task P4 preview)

Once tools resolve, add a Fabro **hook** (`fabro-hooks`) at run-start → `pixl_wakeup`, stage-end →
`pixl_diary_write`, so memory isn't left to the model's discretion. This reproduces pixl-ai's BrainClient.

## What's blocked (needs owner input)

1. **pixl-kb running** (`cd pixl-kb && make dev-api`) + a **workspace id** to target.
2. **A model key** (or Claude subscription via the agent) so the agent node can actually run and call the tool.
3. **Transport decision** for sandboxed/cloud runs (A vs B) — C is fine for the local proof.
