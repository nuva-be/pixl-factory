# Connectors (Linear · Slack · Notion)

> Wire third-party tools into pixl-factory workflows the same way pixl-kb was
> wired: as **MCP servers registered in the host's Claude Code config**. A
> `backend="acp"` node drives Claude Code over ACP, and the ACP session
> inherits the host's MCP servers — so the agent sees `mcp__<connector>__*`
> tools and can call them. **No engine code changes.** This mirrors
> `docs/pixl/proof-kb-memory.md` exactly.

## Architecture (why there's no engine code)

```
┌──────────────────────────────────────────────────────────────┐
│ host machine (where `claude` is authenticated)                │
│                                                                │
│   Claude Code config (user scope)                              │
│     ├─ pixl-crew plugin        (agents / skills)               │
│     ├─ pixl-kb  MCP            → mcp__pixl-kb__*                │
│     ├─ linear   MCP  (connector) → mcp__linear__*              │
│     ├─ slack    MCP  (connector) → mcp__slack__*               │
│     └─ notion   MCP  (connector) → mcp__notion__*              │
│                                                                │
│   fabro run … (environment id="local", provider="local")       │
│     └─ backend="acp" node                                      │
│          └─ npx --yes @zed-industries/claude-code-acp          │
│               └─ inherits ALL of the above ─────────────┘      │
└──────────────────────────────────────────────────────────────┘
```

A connector is just an MCP server registered with `claude mcp add … -s user`.
Because the registration is in **user scope**, every ACP session started by a
`backend="acp"` node inherits it automatically — there is nothing to add to
pixl-factory's Rust, no new node backend, no run-TOML MCP block required for the
local path. The engine stays additive-only; the connector lives entirely in the
host's Claude Code config.

> Trio contract reminder: connectors are a **pixl-os / host** concern (auth,
> connectors, config). The engine (pixl-factory) only drives Claude Code over
> ACP and inherits whatever the host exposes. Keep auth tokens on the host.

## Registration — one command per connector (user scope)

Register on the host where `claude` is authenticated. User scope (`-s user`) is
what makes ACP sessions inherit the server.

### Linear

Linear ships an **official remote MCP server** over SSE — no local process, no
package to install. OAuth happens in the browser on first use.

```bash
claude mcp add --transport sse linear https://mcp.linear.app/sse
```

- **Auth:** OAuth (browser consent on first tool call) — or a Linear **API key**
  if you self-host a local Linear MCP instead of the remote SSE endpoint.
- **Tool prefix the agent sees:** `mcp__linear__*`
  (e.g. `mcp__linear__list_issues`, `mcp__linear__get_issue`,
  `mcp__linear__create_issue`, `mcp__linear__list_teams`).

### Slack

Slack is registered as a local stdio MCP server that authenticates with a Slack
**bot token** (the `xoxb-…` family), plus a team id. Verify the latest package
name before relying on it.

```bash
claude mcp add slack -s user \
  -e SLACK_BOT_TOKEN=<your-slack-bot-token> \
  -e SLACK_TEAM_ID=<your-team-id> \
  -- npx -y <slack-mcp-package>        # package: verify latest
```

- **Auth:** Slack **bot token** from a Slack app with the relevant scopes (e.g.
  `channels:read`, `channels:history`, `chat:write`), plus the workspace
  `SLACK_TEAM_ID`.
- **Tool prefix the agent sees:** `mcp__slack__*`
  (e.g. `mcp__slack__list_channels`, `mcp__slack__post_message`,
  `mcp__slack__get_channel_history`).

> Package note: the reference implementation historically shipped as
> `@modelcontextprotocol/server-slack`; confirm the current, maintained package
> name and its required env vars before wiring it for a client. The
> **registration pattern** above is stable regardless of the package name.

### Notion

Notion offers a hosted MCP endpoint; you can register it over SSE the same way
as Linear. (A local stdio server backed by a Notion **integration token** is the
alternative if you prefer no browser OAuth.)

```bash
# Remote (OAuth in the browser):
claude mcp add --transport sse notion https://mcp.notion.com/sse

# OR local stdio with an integration token (verify latest package):
claude mcp add notion -s user \
  -e NOTION_TOKEN=<your-notion-integration-token> \
  -- npx -y <notion-mcp-package>       # package: verify latest
```

- **Auth:** Notion **integration token** for the local stdio server, or OAuth
  for the hosted SSE endpoint. The integration must be **shared with** each
  page/database you want it to reach (Notion-side share dialog).
- **Tool prefix the agent sees:** `mcp__notion__*`
  (e.g. `mcp__notion__search`, `mcp__notion__fetch`,
  `mcp__notion__create_page`).

## Verify

```bash
claude mcp list
```

You should see `linear`, `slack`, and/or `notion` listed (alongside `pixl-kb`).
A server that fails to connect shows an error here — fix it before running a
workflow, since the ACP node will simply not see the tool and your node will
report `CONNECTOR UNAVAILABLE`.

Remove a connector later with, e.g.:

```bash
claude mcp remove linear -s user
```

## Run a connector workflow

Each connector example is a one-node `backend="acp"` graph on the **local**
environment (so the ACP adapter runs on the host where the MCP servers and
`claude` auth live), following the same `Start → node → Exit` shape as
`examples/pixl/kb-recall.fabro`.

```bash
fabro run examples/pixl/connector-linear.toml --auto-approve
fabro run examples/pixl/connector-notion.toml --auto-approve
```

Each node prompt tells the agent to call the connector's tool and report the
result, with a graceful single-line fallback (`CONNECTOR UNAVAILABLE: <name>`)
if the tool isn't registered — so a run never hard-fails just because a
connector is missing on the host.

- `examples/pixl/connector-linear.fabro` — list my assigned Linear issues.
- `examples/pixl/connector-notion.fabro` — search a Notion workspace.

(A Slack example follows the identical shape — copy `connector-linear.fabro`,
swap the prompt to call `mcp__slack__*`, and point its `.toml` at the new
`.fabro`. The Slack MCP package must be registered first per above.)

## Chaining a connector with a native kb node

The real payoff is composing a connector (`backend="acp"`) with a native pixl-kb
node (`backend="kb"`, see `docs/pixl/kb-node.md`) in one graph: pull data from a
SaaS tool with the agent, then **persist a summary to pixl-kb deterministically**
— no second LLM call, guaranteed write.

```dot
digraph LinearToKb {
  graph [goal="Pull Linear issues via ACP, then persist a summary to pixl-kb"]

  start  [shape=Mdiamond, label="Start"]
  exit   [shape=Msquare,  label="Exit"]

  // 1. Agent pulls from Linear over ACP (inherits mcp__linear__*).
  pull [
    label="Pull",
    backend="acp",
    acp.command="npx --yes @zed-industries/claude-code-acp",
    prompt="Call mcp__linear__list_issues for my assigned issues and write a 3-bullet summary of their identifiers and titles. If no mcp__linear__* tool exists, output 'CONNECTOR UNAVAILABLE: linear'."
  ]

  // 2. Native kb node persists that summary — deterministic, LLM-free.
  //    The previous node's response is sent as the document body.
  save [
    label="Save",
    backend="kb",
    kb.tool="pixl_add_document",
    kb.workspace="<workspace-id>",
    kb.token="<jwt>"          // back with a fabro vault secret in production
  ]

  start -> pull -> save -> exit
}
```

This is the inverse of `examples/pixl/kb-node.fabro` (which does
`recall(kb) → report(acp)`): here the agent step runs first to gather external
data, then a guaranteed kb write closes the loop. See `docs/pixl/kb-node.md` for
the full `backend="kb"` attribute reference (`kb.tool`, `kb.workspace`,
`kb.args`, `kb.endpoint`, `kb.token`) and the wire protocol.

> Why split the work across two backends: the `acp` step is where indeterministic
> tool use belongs (the agent *decides* how to query Linear), while the `kb` step
> is where you want a guaranteed, cost-free persist. Same pattern, any connector:
> `<connector over acp> → <pixl_add_document / pixl_diary_write over kb>`.
