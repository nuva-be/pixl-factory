# pixl-factory — product roadmap

The product: **own the engine, embed our stack, make it a daily-driver tool** you
run your own project work in — on your Claude subscription — and eventually deploy
to your VPS. This roadmap captures what's shipped and what's next, grouped into
phases by priority.

> Status legend: ✅ shipped · ▶ doing (in flight) · ⏳ next · 🔭 later

---

## Done (foundation — shipped & pushed)

- ✅ Hard fork → `nuva-be/pixl-factory`, full rebrand (app + marketing): green-on-near-black, sharp corners, `>_` mark, Space Grotesk / JetBrains Mono.
- ✅ pixl-kb wired as MCP server for `backend="acp"` nodes (P0).
- ✅ ACP drives Claude Code **on the subscription** with the full pixl-crew plugin (14 agents, ~117 skills) (P2).
- ✅ 14 curated crew skills bundled for the native agent (`/commit`, `/code-review`, …) (P1).
- ✅ Auto-memory hook: every run writes a run report into pixl-kb (P4).
- ✅ **Native `backend="kb"` DAG node** — deterministic, LLM-free kb recall/write as a graph step (the engine extension we own).
- ✅ Run-detail **Memory tab** + embedded **Knowledge section** (Search / Documents / Graph), shadcn, monochrome + green.
- ✅ Linear/Slack/Notion **connector** docs + example workflows (ACP-inherited MCP).

---

## Phase 1 — Daily-driver tool (make it usable, not just live) · **P0**

The core ask: drive real project work in the UI, not just watch runs.

- ▶ **Knowledge polish** (in flight): kb connection config moved to **Settings → Knowledge Base**; Graph tab fixed to query by `entity_name`; Documents lazy-loads.
- ⏳ **Wire chat on the subscription**: the playground/chat path needs a model. Route it through ACP (Claude Code subscription) so chat works with no API key — or an OpenRouter/API-key fallback. *Acceptance: open `/chats`, send a message, get a streamed reply with no API key configured.*
- ⏳ **One-click run from the UI**: a "New run" flow that picks a workflow (or repo + goal) and launches it; surface the live DAG + outputs inline. *Acceptance: launch a `.fabro` from `/start` without touching the CLI.*
- ⏳ **Workflow library**: ship the `examples/pixl/*` (kb-node, acp-subscription, connectors) as in-UI templates you can run or clone. *Acceptance: a template gallery on `/start`.*
- ⏳ **Files-first run view**: make "see what changed" a first-class, fast view (diff + file tree) per run.

## Phase 2 — Simplify & lock-in (the Linear feel) · **P1**

Sober, monochrome, green-only accent; bake our defaults; hide the sprawl.

- ⏳ **Trim the shell**: reduce top-level nav to the essentials (Chat · Runs · Knowledge · New), demote the rest. Single command palette (⌘K).
- ⏳ **Incremental shadcn re-skin** of the remaining pages (runs, settings, chat) to the monochrome+green system already used in Knowledge.
- ⏳ **Locked defaults + first-run**: bake subscription auth, local sandbox, kb wired, feen workspace — an onboarding that sets the config once and hides advanced settings behind a "pro" toggle.
- 🔭 **Branding finish**: rebrand the Mintlify docs site; tighten the marketing site.

## Phase 3 — Memory & intelligence depth · **P2**

Make compounding memory the differentiator, end to end.

- ⏳ **Full memory lifecycle hooks**: `pixl_wakeup` at run-start + `pixl_diary_write` at stage-end (extend the run-complete report) so memory is automatic across the run, not just at the end.
- ⏳ **Richer Memory tab**: show what each run *recalled* vs *wrote*, with a timeline (kb activity ring buffer) — not just a live search.
- ⏳ **kb write nodes in templates**: ship graphs that recall context → run a crew agent → write the result back to kb (`backend="kb"` with `pixl_add_document`).
- 🔭 **Knowledge graph view**: a real graph visualization in the Knowledge section (entities + relations), beyond the textual list.

## Phase 4 — Connectors, live · **P2**

- ⏳ **Register + prove** Linear / Slack / Notion MCP for ACP nodes (we have docs + examples; make them work end-to-end on this host).
- ⏳ **Connector status in Settings → Integrations**: show which MCP servers are registered + a "test" per connector.
- 🔭 **Expand**: Gmail, Calendar, GitHub, Supabase, Sentry, PostHog (the MCP suite) as one-click connectors.

## Phase 5 — Authoring experience · **P3**

- 🔭 **Visual DAG builder**: turn the read-only graph viewer into a drag-to-edit builder (nodes = api/acp/kb, edges, attrs).
- 🔭 **NL → workflow**: lean on the playground to generate `.fabro` graphs from a sentence (needs chat/LLM wired first).
- 🔭 **Node palette** for our backends: first-class `kb` / `acp` / `api` node cards with attr forms.

## Phase 6 — Deploy to your VPS · **P3 (the finish line for self-use)**

- ⏳ **Containerize**: Docker for pixl-factory + pixl-kb (the fork already has `docker/`).
- ⏳ **Subscription on a headless box**: document + script the Claude Code device-login path (preferred) vs API-key mode.
- ⏳ **Domain + TLS + auth hardening**: real auth (beyond the dev token), HTTPS, persistence/backups for SlateDB + kb SQLite.
- ⏳ **One-command deploy**: `make deploy` to the VPS.
- *Acceptance: pixl-factory reachable at your domain, you log in, run a task against one of your repos on the subscription.*

---

## Decisions still open

1. **Chat provider** — subscription (ACP) vs API-key/OpenRouter for the first cut. (Phase 1)
2. **Nav structure** — exactly which items stay top-level after the trim. (Phase 2)
3. **Deploy target** — which VPS / OS, and subscription-login vs API-key on the box. (Phase 6)

## Sequencing recommendation

Finish **Phase 1** (it's what makes it a tool you'd actually use today), then **Phase 2**
(the look + lock-in you asked for), then **Phase 3–4** in parallel (memory + connectors),
and **Phase 6** when you're ready to run it from your VPS. Phase 5 (visual authoring) is
the ambitious stretch once the daily loop is solid.
