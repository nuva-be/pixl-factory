import { useCallback, useEffect, useState } from "react";
import { useParams } from "react-router";

import { EmptyState, ErrorState, LoadingState } from "../components/state";
import { useRun } from "../lib/queries";

const LS_ENDPOINT = "pixl.kb.endpoint";
const LS_TOKEN = "pixl.kb.token";
const LS_WORKSPACE = "pixl.kb.workspace";

const DEFAULT_ENDPOINT = "http://localhost:8421/api/mcp/call";
const DEFAULT_WORKSPACE = "42e3f37a-bfe2-41e2-9ea2-e05b24586b46";

interface MemoryResult {
  title: string;
  content: string;
  source: string;
  score: number;
}

type SearchState =
  | { kind: "idle" }
  | { kind: "loading" }
  | { kind: "error"; message: string; status?: number }
  | { kind: "results"; items: MemoryResult[] };

function readLS(key: string, fallback: string): string {
  try {
    return localStorage.getItem(key) ?? fallback;
  } catch {
    return fallback;
  }
}

function writeLS(key: string, value: string): void {
  try {
    localStorage.setItem(key, value);
  } catch {
    // ignore quota errors
  }
}

export default function RunMemory() {
  const { id } = useParams();
  // useRun so the tab can react to run lifecycle (mirrors run-logs.tsx pattern)
  useRun(id);

  const [endpoint, setEndpoint] = useState(() =>
    readLS(LS_ENDPOINT, DEFAULT_ENDPOINT),
  );
  const [token, setToken] = useState(() => readLS(LS_TOKEN, ""));
  const [workspace, setWorkspace] = useState(() =>
    readLS(LS_WORKSPACE, DEFAULT_WORKSPACE),
  );
  const [query, setQuery] = useState("overview");
  const [state, setState] = useState<SearchState>({ kind: "idle" });

  // Persist settings to localStorage whenever they change
  useEffect(() => writeLS(LS_ENDPOINT, endpoint), [endpoint]);
  useEffect(() => writeLS(LS_TOKEN, token), [token]);
  useEffect(() => writeLS(LS_WORKSPACE, workspace), [workspace]);

  const search = useCallback(async () => {
    if (!query.trim()) return;
    setState({ kind: "loading" });
    try {
      const res = await fetch(endpoint, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          ...(token ? { Authorization: `Bearer ${token}` } : {}),
          "X-Workspace-Id": workspace,
        },
        body: JSON.stringify({
          name: "pixl_search",
          arguments: { query: query.trim(), workspace_id: workspace },
        }),
      });

      if (!res.ok) {
        const text = await res.text().catch(() => "");
        setState({
          kind: "error",
          status: res.status,
          message: text || res.statusText || "Request failed",
        });
        return;
      }

      const json = (await res.json()) as {
        content?: { type: string; text: string }[];
      };

      const textEntry = json.content?.find((c) => c.type === "text");
      if (!textEntry) {
        setState({ kind: "results", items: [] });
        return;
      }

      let items: MemoryResult[] = [];
      try {
        const parsed = JSON.parse(textEntry.text) as unknown;
        if (Array.isArray(parsed)) {
          items = parsed.filter(
            (x): x is MemoryResult =>
              x !== null &&
              typeof x === "object" &&
              "title" in x &&
              "content" in x,
          );
        }
      } catch {
        setState({
          kind: "error",
          message: `Could not parse pixl-kb response: ${textEntry.text.slice(0, 120)}`,
        });
        return;
      }

      setState({ kind: "results", items });
    } catch (err) {
      setState({
        kind: "error",
        message: err instanceof Error ? err.message : "Network error",
      });
    }
  }, [endpoint, token, workspace, query]);

  return (
    <div className="space-y-4">
      {/* Header */}
      <p className="text-sm text-fg-2">
        Live recall from pixl-kb. Every run also auto-writes a run report here
        via the memory hook.
      </p>

      {/* Settings row */}
      <div className="rounded-md border border-line bg-panel-alt p-3">
        <p className="mb-2 text-xs font-medium text-fg-3">Connection</p>
        <div className="flex flex-wrap gap-2">
          <label className="flex flex-col gap-1">
            <span className="text-xs text-fg-muted">Endpoint</span>
            <input
              type="text"
              value={endpoint}
              onChange={(e) => setEndpoint(e.target.value)}
              className="w-72 bg-navy-950 border border-line px-2 py-1 text-sm text-fg font-mono focus:outline-none focus:border-teal-500"
              placeholder={DEFAULT_ENDPOINT}
              aria-label="pixl-kb MCP endpoint"
            />
          </label>
          <label className="flex flex-col gap-1">
            <span className="text-xs text-fg-muted">Bearer token</span>
            <input
              type="password"
              value={token}
              onChange={(e) => setToken(e.target.value)}
              className="w-52 bg-navy-950 border border-line px-2 py-1 text-sm text-fg font-mono focus:outline-none focus:border-teal-500"
              placeholder="sk-…"
              aria-label="pixl-kb bearer token"
            />
          </label>
          <label className="flex flex-col gap-1">
            <span className="text-xs text-fg-muted">Workspace ID</span>
            <input
              type="text"
              value={workspace}
              onChange={(e) => setWorkspace(e.target.value)}
              className="w-72 bg-navy-950 border border-line px-2 py-1 text-sm text-fg font-mono focus:outline-none focus:border-teal-500"
              placeholder={DEFAULT_WORKSPACE}
              aria-label="pixl-kb workspace ID"
            />
          </label>
        </div>
      </div>

      {/* Query row */}
      <div className="flex items-end gap-2">
        <label className="flex flex-1 flex-col gap-1">
          <span className="text-xs text-fg-muted">Search query</span>
          <input
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") void search();
            }}
            className="bg-navy-950 border border-line px-2 py-1 text-sm text-fg focus:outline-none focus:border-teal-500"
            placeholder="overview"
            aria-label="Memory search query"
          />
        </label>
        <button
          type="button"
          onClick={() => void search()}
          disabled={state.kind === "loading"}
          className="border border-teal-500 px-3 py-1 text-sm font-medium text-teal-500 transition-colors hover:bg-overlay disabled:opacity-50 focus-visible:outline-2 focus-visible:outline-offset-1 focus-visible:outline-teal-500"
        >
          {state.kind === "loading" ? "Searching…" : "Search"}
        </button>
      </div>

      {/* Results */}
      <MemoryBody state={state} />
    </div>
  );
}

function MemoryBody({ state }: { state: SearchState }) {
  if (state.kind === "idle") {
    return null;
  }
  if (state.kind === "loading") {
    return <LoadingState label="Searching pixl-kb…" />;
  }
  if (state.kind === "error") {
    const hint =
      "set a pixl-kb bearer token; ensure kb CORS allows this origin";
    const detail = state.status
      ? `HTTP ${state.status}: ${state.message}`
      : state.message;
    return (
      <ErrorState
        title="pixl-kb search failed"
        description={`${detail} — ${hint}`}
      />
    );
  }
  if (state.items.length === 0) {
    return (
      <EmptyState
        title="No memory found"
        description="Try a different query or check your connection settings above."
      />
    );
  }
  return (
    <div className="space-y-3">
      {state.items.map((item, i) => (
        <MemoryCard key={`${item.source}-${i}`} item={item} />
      ))}
    </div>
  );
}

function MemoryCard({ item }: { item: MemoryResult }) {
  const snippet =
    item.content.length > 200
      ? `${item.content.slice(0, 200).trimEnd()}…`
      : item.content;

  return (
    <div className="rounded-md border border-line bg-panel-alt p-4 hover:bg-overlay transition-colors">
      <p className="text-sm font-medium text-fg">{item.title}</p>
      <p className="mt-1 text-sm text-fg-2 leading-relaxed">{snippet}</p>
      <div className="mt-2 flex items-center gap-3">
        <span className="font-mono text-xs text-fg-3 truncate">{item.source}</span>
        <span className="shrink-0 font-mono text-xs text-fg-muted">
          score {item.score.toFixed(3)}
        </span>
      </div>
    </div>
  );
}
