# Proof: pixl-factory drives Claude Code on the subscription (no API key)

> Verified 2026-06-19 with the from-source pixl-factory binary. This is the keystone of the pixl integration:
> work nodes run on the Claude subscription via ACP, so there is no per-token API cost and Claude Code owns
> its own tools (crew plugin + MCP).

## What was proven

```
fabro run examples/pixl/acp-subscription.toml --auto-approve
  Sandbox: local (ready in 0ms)
  ✓ Start   0ms
  ✓ Recall  23s
  ✓ Exit    0ms
  Status: SUCCEEDED
  Output: Hello from Claude Code! Working directory: /Users/hamzamounir/code/nuva/fabro
```

- **Engine**: our rebranded pixl-factory binary (built from the fork).
- **Backend**: `backend="acp"`, `acp.command="npx --yes @zed-industries/claude-code-acp"`.
- **Auth**: install ran with `--skip-llm` — **no Anthropic API key**. Claude Code authenticated with the
  developer's subscription. The ACP backend lets the external agent own model auth.
- **Sandbox**: `local` environment, so the ACP adapter runs on the host (where `claude` is logged in). Docker
  failed earlier with exit 127 because node/claude do not exist inside the container.

## Setup that made it work (recipe)

1. Build the binary: `cargo build` (from the fork).
2. Configure an instance without an LLM key:
   `fabro install --non-interactive --skip-llm --github-strategy token --github-username <you>`
3. Select a local environment per run (sandbox provider is per-environment now; `[run.sandbox]` is legacy):
   ```toml
   [run.environment]
   id = "local"
   [environments.local]
   provider = "local"
   ```
4. `fabro run <config>.toml --auto-approve`.

## What this unblocks

- **P2 (crew deep)**: the ACP→Claude Code path works; the host's Claude Code already loads the pixl-crew plugin,
  so crew agents/skills come along for free in ACP work nodes.
- **P0 (kb)**: register pixl-kb as an MCP server in Claude Code (`claude mcp add pixl-kb …` or a project
  `.mcp.json`), and the same ACP session gains `pixl_search`/`pixl_wakeup`/`pixl_diary_*`.
- **Cost**: work runs on the subscription; only cheap `decide`/`judge` nodes need an API key (or OpenRouter).

## Next step

Layer pixl-kb MCP into the ACP Claude Code session and have the Recall node call `pixl_wakeup` / `pixl_search`,
then assert the `mcp__pixl-kb__*` tool calls appear in the run events. (Needs a kb workspace id.)
