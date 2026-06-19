# Models & providers — subscription vs OpenRouter

Two distinct LLM paths in pixl-factory. Knowing which is which removes the
"I don't have a model" confusion.

## The split (important)

| Path | Powered by | Used for | Needs |
|---|---|---|---|
| `backend="acp"` nodes | **Claude subscription** (via Claude Code / ACP) | workflow agent nodes, the crew | a Claude Code login on the host — **no API key** |
| Chat / playground / `backend="api"` nodes | **API providers** (OpenRouter, Anthropic, …) | chat, prompt nodes, api workflow nodes | an **API key** |

The subscription is consumed *through Claude Code*, so it can only drive ACP
nodes. Chat and `api` nodes call provider HTTP APIs directly and need a key —
the subscription cannot back them. This is why a `--skip-llm` install shows
**no model**: no API provider is configured, so `catalog.default_model()` is empty.

## Enable OpenRouter (one key → many models)

OpenRouter ships as a built-in catalog provider (`lib/crates/fabro-model/src/catalog/providers/openrouter.toml`,
disabled by default) with Claude Opus 4.7 / Sonnet 4.6 / Haiku 4.5, GPT-5.4, and more.

1. Enable it in `~/.fabro/settings.toml`:
   ```toml
   [llm.providers.openrouter]
   enabled = true
   ```
2. Supply the key — either:
   - env on the server: `OPENROUTER_API_KEY=sk-or-...` (used by the running instance), or
   - a vault secret (durable across restarts): Settings → Secrets, name `OPENROUTER_API_KEY`
     (or `fabro provider login openrouter`).
3. Restart the server. OpenRouter then shows **configured** with its models, and chat works.

**Verified (2026-06-19):** OpenRouter configured, 17 models, default `anthropic/claude-sonnet-4-6`.
A `backend="api"` node ran through OpenRouter and returned output in ~1s ($0.03).

## Durability note

Passing `OPENROUTER_API_KEY` (and the kb `PIXL_KB_TOKEN`) as **server env vars** works
for the running process but does not survive a restart. For a persistent / deployed
setup, store them as **vault secrets** (Settings → Secrets) so the server picks them up
on every boot. This is a Phase-1 polish item.

## Recommendation

- **Chat + api nodes** → OpenRouter (Claude Sonnet 4.6 default; cheap option: Gemini/Haiku).
- **Crew / agent workflow nodes** → keep `backend="acp"` on the subscription (no key, full crew).
- Mix both in one graph: recall (kb) → reason (acp, subscription) → summarize (api, OpenRouter) → write (kb).
