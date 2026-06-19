/**
 * Pixl example workflow templates embedded as frontend constants.
 * Each template includes the full DOT/.fabro graph source copied verbatim
 * from examples/pixl/*.fabro so the gallery can run them via the inline-DOT
 * manifest path (same approach as the Playground's "Run for real").
 */

export interface WorkflowTemplate {
  /** Stable slug used as React key and manifest identifier */
  id: string;
  /** Human-readable display name */
  name: string;
  /** One-line description shown on the card */
  description: string;
  /** Backend badge labels (lowercase) */
  backends: string[];
  /** Full .fabro DOT graph source */
  dot: string;
  /** Filename of the sibling .toml (used in copy-command fallback) */
  tomlFile: string;
}

export const PIXL_TEMPLATES: WorkflowTemplate[] = [
  {
    id: "acp-subscription",
    name: "ACP Subscription",
    description:
      "Drive Claude Code over ACP using your Claude Max/Pro subscription — no API key required.",
    backends: ["acp"],
    tomlFile: "acp-subscription.toml",
    dot: `// pixl-factory keystone: drive Claude Code over ACP on the Claude subscription.
// Run with the sibling acp-subscription.toml (selects the local environment so the
// ACP adapter runs on the host where \`claude\` is authenticated):
//   fabro run examples/pixl/acp-subscription.toml --auto-approve
//
// No LLM API key required — the ACP backend lets Claude Code own its model auth
// (your Max/Pro subscription). Proven 2026-06-19.
digraph AcpSubscription {
  graph [goal="Drive Claude Code over ACP on the Claude subscription"]

  start  [shape=Mdiamond, label="Start"]
  exit   [shape=Msquare,  label="Exit"]

  recall [
    label="Recall",
    backend="acp",
    acp.command="npx --yes @zed-industries/claude-code-acp",
    prompt="Say hello from Claude Code, then run the shell command \`pwd\` and report the directory. Keep your reply under 3 lines."
  ]

  start -> recall -> exit
}`,
  },
  {
    id: "kb-recall",
    name: "KB Recall",
    description:
      "Agent recalls documents from pixl-kb via an MCP tool inside a Claude Code ACP session.",
    backends: ["acp"],
    tomlFile: "kb-recall.toml",
    dot: `digraph KbRecall {
  graph [goal="Prove pixl-kb tools work inside the ACP Claude Code session"]

  start  [shape=Mdiamond, label="Start"]
  exit   [shape=Msquare,  label="Exit"]

  recall [
    label="Recall",
    backend="acp",
    acp.command="npx --yes @zed-industries/claude-code-acp",
    prompt="You have a pixl-kb MCP server connected. Call the tool mcp__pixl-kb__pixl_search with query \\"overview\\" (the workspace is preset via env). Report exactly: (1) whether the tool call succeeded, (2) how many results returned, (3) the title of the first result. If the pixl-kb tool is not available, say 'PIXL-KB TOOL UNAVAILABLE'. Keep it under 6 lines."
  ]

  start -> recall -> exit
}`,
  },
  {
    id: "kb-node",
    name: "KB Node",
    description:
      "Native pixl-kb recall node (no LLM) feeds raw results to an ACP agent that summarizes them.",
    backends: ["kb", "acp"],
    tomlFile: "kb-node.toml",
    dot: `// Native pixl-kb node — backend="kb".
//
// The "recall" node hits the pixl-kb MCP gateway directly (no LLM, no agent):
// it sends the node prompt as the search query and returns the tool's text as
// the node response, which the "report" node then summarizes. This is a
// deterministic, LLM-free graph step.
//
//   fabro run examples/pixl/kb-node.toml --auto-approve
//
// Set kb.workspace to a real workspace and deliver a pixl-kb bearer token via
// the kb.token attr (best as a fabro vault secret) — e.g. kb.token="<jwt>" on
// the recall node. PIXL_KB_TOKEN env also works when the run executor inherits
// the environment (CLI-local runs). kb.endpoint defaults to
// http://localhost:8421/api/mcp/call.
digraph KbNode {
  graph [goal="Recall from pixl-kb with a native kb node, then summarize"]

  start  [shape=Mdiamond, label="Start"]
  exit   [shape=Msquare,  label="Exit"]

  recall [
    label="Recall",
    backend="kb",
    kb.tool="pixl_search",
    kb.workspace="42e3f37a-bfe2-41e2-9ea2-e05b24586b46",
    prompt="overview"
  ]

  report [
    label="Report",
    backend="acp",
    acp.command="npx --yes @zed-industries/claude-code-acp",
    prompt="The previous step recalled documents from pixl-kb. Summarize the recalled titles in 3 bullet points. If the input is empty, say 'NO RESULTS'."
  ]

  start -> recall -> report -> exit
}`,
  },
  {
    id: "connector-linear",
    name: "Linear Connector",
    description:
      "Pull assigned Linear issues via a Linear MCP tool registered in your Claude Code session.",
    backends: ["acp"],
    tomlFile: "connector-linear.toml",
    dot: `// pixl-factory connector: Linear over ACP.
//
// The "linear" node drives Claude Code over ACP. Claude Code inherits the
// host's MCP servers, so if a Linear MCP server is registered (see
// docs/pixl/connectors.md), the agent sees mcp__linear__* tools and can call
// them. No engine code — the connector is a host-side MCP registration that
// backend="acp" nodes inherit automatically, exactly like the pixl-kb path.
//
//   fabro run examples/pixl/connector-linear.toml --auto-approve
//
// Register first (user scope, inherited by ACP sessions):
//   claude mcp add --transport sse linear https://mcp.linear.app/sse
// Verify:  claude mcp list   (should list "linear")
digraph ConnectorLinear {
  graph [goal="Pull my assigned Linear issues via a Linear MCP tool over ACP"]

  start  [shape=Mdiamond, label="Start"]
  exit   [shape=Msquare,  label="Exit"]

  linear [
    label="Linear",
    backend="acp",
    acp.command="npx --yes @zed-industries/claude-code-acp",
    prompt="You have a Linear MCP server connected (tools named mcp__linear__*). Call the tool that lists issues (e.g. mcp__linear__list_issues) for the issues assigned to me. Report exactly: (1) whether the tool call succeeded, (2) how many issues returned, (3) the identifier and title of the first issue. If no mcp__linear__* tool is available, reply on a single line: 'CONNECTOR UNAVAILABLE: linear'. Keep it under 6 lines."
  ]

  start -> linear -> exit
}`,
  },
  {
    id: "connector-notion",
    name: "Notion Connector",
    description:
      "Search a Notion workspace via a Notion MCP tool registered in your Claude Code session.",
    backends: ["acp"],
    tomlFile: "connector-notion.toml",
    dot: `// pixl-factory connector: Notion over ACP.
//
// The "notion" node drives Claude Code over ACP. Claude Code inherits the
// host's MCP servers, so if a Notion MCP server is registered (see
// docs/pixl/connectors.md), the agent sees mcp__notion__* tools and can call
// them. No engine code — the connector is a host-side MCP registration that
// backend="acp" nodes inherit automatically, exactly like the pixl-kb path.
//
//   fabro run examples/pixl/connector-notion.toml --auto-approve
//
// Register first (user scope, inherited by ACP sessions):
//   claude mcp add --transport sse notion https://mcp.notion.com/sse
// Verify:  claude mcp list   (should list "notion")
digraph ConnectorNotion {
  graph [goal="Search a Notion workspace via a Notion MCP tool over ACP"]

  start  [shape=Mdiamond, label="Start"]
  exit   [shape=Msquare,  label="Exit"]

  notion [
    label="Notion",
    backend="acp",
    acp.command="npx --yes @zed-industries/claude-code-acp",
    prompt="You have a Notion MCP server connected (tools named mcp__notion__*). Call the search tool (e.g. mcp__notion__search) for the query \\"roadmap\\". Report exactly: (1) whether the tool call succeeded, (2) how many results returned, (3) the title of the first result. If no mcp__notion__* tool is available, reply on a single line: 'CONNECTOR UNAVAILABLE: notion'. Keep it under 6 lines."
  ]

  start -> notion -> exit
}`,
  },
];
