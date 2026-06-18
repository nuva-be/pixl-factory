# Native pixl-kb node (`backend="kb"`)

A first-class DAG node type that calls pixl-kb directly — **no LLM, no agent loop**.
Where `backend="acp"` drives Claude Code (which *may* call a kb MCP tool if it
chooses), `backend="kb"` *is* the kb call: a deterministic, LLM-cost-free graph
step that recalls from or writes to pixl-kb and returns the tool's text as the
node response. Downstream nodes and routing read that response like any other.

This is the pixl-factory DAG extension over upstream Fabro (which has only
`api` and `acp` backends).

## Usage

```dot
recall [
  label="Recall",
  backend="kb",
  "kb.tool"="pixl_search",
  "kb.workspace"="<workspace-id>",
  prompt="overview"           // becomes {"query": "overview"}
]
```

```bash
PIXL_KB_TOKEN=<jwt> fabro run examples/pixl/kb-node.toml --auto-approve
```

## Node attributes

| Attribute | Default | Meaning |
|---|---|---|
| `kb.tool` | `pixl_search` | MCP tool name (`pixl_search`, `pixl_add_document`, `pixl_wakeup`, `pixl_diary_write`, …) |
| `kb.endpoint` | `http://localhost:8421/api/mcp/call` | pixl-kb MCP gateway URL |
| `kb.workspace` | — | Workspace id → `X-Workspace-Id` header **and** injected into args as `workspace_id` |
| `kb.args` | — | JSON object of tool arguments. When absent, the node prompt is sent as `{"query": <prompt>}` |
| `kb.token` | `$PIXL_KB_TOKEN` | Bearer token for the gateway |

`kb.args` takes precedence over the prompt and is not overwritten — an explicit
`workspace_id` inside `kb.args` wins over `kb.workspace`.

## Wire protocol

```
POST <kb.endpoint>
  Authorization: Bearer <token>
  X-Workspace-Id: <workspace>
  { "name": "<kb.tool>", "arguments": { ... } }
→ 200 { "content": [ { "type": "text", "text": "..." } ] }
```

The backend joins the `content[].text` parts into the node response. A 4xx
(bad tool/args/auth) is a terminal validation error; a 5xx is a transient
(retryable) handler error.

## Implementation

- Enum variant: `fabro-types/src/llm_backend.rs` → `AgentBackend::Kb`
- Attr accessors: `fabro-types/src/graph.rs` → `kb_tool_attr` / `kb_endpoint_attr` / `kb_workspace_attr` / `kb_args_attr` / `kb_token_attr`
- Backend: `fabro-workflow/src/handler/llm/kb.rs` → `AgentKbBackend` (`CodergenBackend`)
- Dispatch: `fabro-workflow/src/handler/llm/router.rs` → `AgentBackend::Kb => self.kb.run(...)`
- Routing guard: `routing.rs` rejects `backend="kb"` on prompt (one-shot) nodes
- HTTP via the existing `fabro-http` client (reqwest)

`AgentKbBackend` is constructed inside `BackendRouter::new`, so no caller
signatures changed — the addition is contained to the backend dispatch layer.

## Proven (2026-06-19)

```
fabro run examples/pixl/kb-node.toml --auto-approve   # (kb.token set on the recall node)
  ✓ Start   0ms
  ✓ Recall  563ms        ← native kb node: HTTP 200, 10 results from pixl-kb
  ✓ Exit    0ms
  Status: SUCCEEDED
```

The recall node returned real `pixl_search` results from the feen workspace —
including `doc-925b8649`, the run report written earlier by the P4 memory hook
(the kb node reads what the kb hook wrote). At **563ms** with no LLM, versus
~13s for the equivalent `acp` node, the cost/latency win is the whole point.

Token delivery (proven): the run executor runs with a scrubbed environment, so
the reliable path is the `kb.token` node attribute (back it with a fabro vault
secret in production). `$PIXL_KB_TOKEN` works only when the executor inherits
the environment.

## Why a native node (vs. the ACP path)

The `acp` path is powerful but indeterministic and costs LLM tokens: the agent
*decides* whether to call kb. For pipeline steps that must *always* recall or
*always* persist — a wakeup at run-start, a diary write at stage-end, a
retrieval feeding a downstream prompt — a native kb node is cheaper, faster,
and guaranteed. It composes with `acp` nodes in the same graph (see
`examples/pixl/kb-node.fabro`: `recall(kb) → report(acp)`).
