#!/usr/bin/env python3
"""pixl-factory memory hook — write a run report into pixl-kb.

Invoked by pixl-factory (Fabro) as a `command` hook on `run_complete` (or
`stage_complete`). Reads the FABRO_* env vars + the context JSON file at
$FABRO_HOOK_CONTEXT, then dispatches `pixl_add_document` into the workspace
named by PIXL_KB_WORKSPACE_ID. Run it via the kb's uv env, e.g.:

  [[hooks]]
  event = "run_complete"
  command = "PIXL_KB_WORKSPACE_ID=<ws> uv --directory <pixl-kb> run python <this-file>"

Always exits 0 — kb being down must never fail a run (graceful degradation).
"""

from __future__ import annotations

import asyncio
import json
import os
import sys


def _load_context() -> dict:
    path = os.environ.get("FABRO_HOOK_CONTEXT")
    if path and os.path.exists(path):
        try:
            with open(path) as fh:
                return json.load(fh)
        except Exception:
            return {}
    return {}


async def _main() -> int:
    ws = os.environ.get("PIXL_KB_WORKSPACE_ID")
    if not ws:
        print("kb-memory-hook: no PIXL_KB_WORKSPACE_ID set; skipping", file=sys.stderr)
        return 0

    event = os.environ.get("FABRO_EVENT", "?")
    run_id = os.environ.get("FABRO_RUN_ID", "?")
    workflow = os.environ.get("FABRO_WORKFLOW", "?")
    node_id = os.environ.get("FABRO_NODE_ID", "")
    ctx = _load_context()

    goal = ctx.get("goal") or (ctx.get("run") or {}).get("goal") or ""
    status = ctx.get("status") or ctx.get("outcome") or "completed"
    nodes = ctx.get("completed_nodes") or ctx.get("nodes") or []

    lines = [
        f"# Run report — {workflow}",
        "",
        f"- run_id: `{run_id}`",
        f"- event: `{event}`",
        f"- status: {status}",
    ]
    if node_id:
        lines.append(f"- node: `{node_id}`")
    if goal:
        lines.append(f"- goal: {goal}")
    if nodes:
        lines.append(f"- nodes: {', '.join(str(n) for n in nodes)}")
    lines += ["", "_Automated run report written by the pixl-factory memory hook._"]
    content = "\n".join(lines)

    dbg = open("/tmp/kb-hook-debug.log", "a")
    dbg.write(f"\n--- hook start event={event} run={run_id} ws={ws[:8]} py={sys.executable}\n")
    try:
        from knowledge_api.mcp.tools import dispatch

        res = await dispatch(
            "pixl_add_document",
            {
                "workspace_id": ws,
                "title": f"Run report — {workflow} ({run_id[:8]})",
                "content": content,
                "tags": ["pixl-factory", "run-report"],
            },
        )
        dbg.write(f"WROTE ok: {[c.text[:80] for c in res]}\n")
        print(f"kb-memory-hook: wrote run report for {run_id} -> workspace {ws[:8]}")
    except Exception as exc:  # never fail the run on a kb problem
        import traceback

        dbg.write(f"FAILED: {exc}\n{traceback.format_exc()}\n")
        print(f"kb-memory-hook: kb write failed (non-fatal): {exc}", file=sys.stderr)
    finally:
        dbg.close()
    return 0


if __name__ == "__main__":
    sys.exit(asyncio.run(_main()))
