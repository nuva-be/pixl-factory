import { useCallback, useState } from "react";
import { Link, useParams } from "react-router";

import { EmptyState, ErrorState, LoadingState } from "../components/state";
import { useRun } from "../lib/queries";
import { kbCall, KbError } from "../lib/kb";

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

export default function RunMemory() {
  const { id } = useParams();
  // useRun so the tab can react to run lifecycle (mirrors run-logs.tsx pattern)
  useRun(id);

  const [query, setQuery] = useState("overview");
  const [state, setState] = useState<SearchState>({ kind: "idle" });

  const search = useCallback(async () => {
    if (!query.trim()) return;
    setState({ kind: "loading" });
    try {
      const parsed = await kbCall<unknown>("pixl_search", { query: query.trim() });
      let items: MemoryResult[] = [];
      if (Array.isArray(parsed)) {
        items = parsed.filter(
          (x): x is MemoryResult =>
            x !== null &&
            typeof x === "object" &&
            "title" in x &&
            "content" in x,
        );
      }
      setState({ kind: "results", items });
    } catch (err) {
      if (err instanceof KbError) {
        setState({
          kind: "error",
          status: err.status,
          message: err.message,
        });
      } else {
        setState({
          kind: "error",
          message: err instanceof Error ? err.message : "Network error",
        });
      }
    }
  }, [query]);

  return (
    <div className="space-y-4">
      {/* Header */}
      <div className="flex items-start justify-between gap-4">
        <p className="text-sm text-fg-2">
          Live recall from pixl-kb. Every run also auto-writes a run report here
          via the memory hook.
        </p>
        <Link
          to="/settings/knowledge"
          className="shrink-0 text-xs text-fg-muted hover:text-teal-500 transition-colors"
        >
          Connection settings →
        </Link>
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
      "check connection settings or set a pixl-kb bearer token";
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
        description="Try a different query or check your connection settings."
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
    <div className="border border-line bg-panel-alt p-4 hover:bg-overlay transition-colors">
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
