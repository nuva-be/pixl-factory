import { useCallback, useEffect, useState } from "react";
import { Settings2 } from "lucide-react";

import { EmptyState, ErrorState } from "../components/state";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "../components/ui/tabs";
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from "../components/ui/card";
import { Input } from "../components/ui/input";
import { Button } from "../components/ui/button";
import { Badge } from "../components/ui/badge";
import { Skeleton } from "../components/ui/skeleton";
import { Separator } from "../components/ui/separator";
import { kbCall, getKbConfig, setKbConfig, KbError } from "../lib/kb";

export function meta() {
  return [{ title: "Knowledge — pixl-factory" }];
}

export const handle = {
  wide: true,
};

// ── Types ──

interface SearchResult {
  title: string;
  content: string;
  source: string;
  score: number;
}

type AsyncState<T> =
  | { kind: "idle" }
  | { kind: "loading" }
  | { kind: "error"; message: string; status?: number }
  | { kind: "data"; value: T };

// ── Error hint ──

const KB_ERROR_HINT = "set a pixl-kb token in config; ensure kb CORS allows this origin";

// ── Config panel ──

function ConfigPanel({
  open,
  onClose,
}: {
  open: boolean;
  onClose: () => void;
}) {
  const cfg = getKbConfig();
  const [endpoint, setEndpoint] = useState(cfg.endpoint);
  const [token, setToken] = useState(cfg.token);
  const [workspace, setWorkspace] = useState(cfg.workspace);

  function save() {
    setKbConfig({ endpoint, token, workspace });
    onClose();
  }

  if (!open) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-start justify-end bg-black/60 backdrop-blur-sm" onClick={onClose}>
      <div
        className="w-full max-w-sm border border-line-strong bg-panel shadow-2xl shadow-black/60 mt-16 mr-4"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="border-b border-line px-4 py-3">
          <p className="text-sm font-medium text-fg">KB Connection</p>
          <p className="mt-0.5 text-xs text-fg-muted">Saved to localStorage</p>
        </div>
        <div className="space-y-3 p-4">
          <div className="space-y-1">
            <label className="text-xs text-fg-muted" htmlFor="kb-endpoint">
              Endpoint
            </label>
            <Input
              id="kb-endpoint"
              value={endpoint}
              onChange={(e) => setEndpoint(e.target.value)}
              placeholder="http://localhost:8421/api/mcp/call"
              className="font-mono text-xs"
              aria-label="pixl-kb MCP endpoint"
            />
          </div>
          <div className="space-y-1">
            <label className="text-xs text-fg-muted" htmlFor="kb-token">
              Bearer token
            </label>
            <Input
              id="kb-token"
              type="password"
              value={token}
              onChange={(e) => setToken(e.target.value)}
              placeholder="sk-…"
              className="font-mono text-xs"
              aria-label="pixl-kb bearer token"
            />
          </div>
          <div className="space-y-1">
            <label className="text-xs text-fg-muted" htmlFor="kb-workspace">
              Workspace ID
            </label>
            <Input
              id="kb-workspace"
              value={workspace}
              onChange={(e) => setWorkspace(e.target.value)}
              placeholder="42e3f37a-bfe2-41e2-9ea2-e05b24586b46"
              className="font-mono text-xs"
              aria-label="pixl-kb workspace ID"
            />
          </div>
        </div>
        <div className="flex justify-end gap-2 border-t border-line px-4 py-3">
          <Button variant="ghost" size="sm" onClick={onClose}>
            Cancel
          </Button>
          <Button variant="primary" size="sm" onClick={save}>
            Save
          </Button>
        </div>
      </div>
    </div>
  );
}

// ── Search tab ──

function SearchTab() {
  const [query, setQuery] = useState("overview");
  const [state, setState] = useState<AsyncState<SearchResult[]>>({ kind: "idle" });
  const [configOpen, setConfigOpen] = useState(false);

  const search = useCallback(async (q: string) => {
    const trimmed = q.trim();
    if (!trimmed) return;
    setState({ kind: "loading" });
    try {
      const raw = await kbCall<unknown>("pixl_search", { query: trimmed });
      const items: SearchResult[] = Array.isArray(raw)
        ? raw.filter(
            (x): x is SearchResult =>
              x !== null && typeof x === "object" && "title" in x && "content" in x,
          )
        : [];
      setState({ kind: "data", value: items });
    } catch (err) {
      if (err instanceof KbError) {
        setState({ kind: "error", message: err.message, status: err.status });
      } else {
        setState({ kind: "error", message: err instanceof Error ? err.message : "Network error" });
      }
    }
  }, []);

  function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
    if (e.key === "Enter") void search(query);
  }

  return (
    <>
      <ConfigPanel open={configOpen} onClose={() => setConfigOpen(false)} />

      <div className="flex items-center gap-2">
        <Input
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Search knowledge base…"
          className="flex-1"
          aria-label="Knowledge search query"
        />
        <Button
          variant="outline"
          size="md"
          onClick={() => void search(query)}
          disabled={state.kind === "loading"}
        >
          {state.kind === "loading" ? "Searching…" : "Search"}
        </Button>
        <Button
          variant="ghost"
          size="icon"
          onClick={() => setConfigOpen(true)}
          aria-label="Open KB connection config"
          title="Connection settings"
        >
          <Settings2 className="size-4" />
        </Button>
      </div>

      <div className="mt-4">
        <SearchResults state={state} />
      </div>
    </>
  );
}

function SearchResults({ state }: { state: AsyncState<SearchResult[]> }) {
  if (state.kind === "idle") {
    return (
      <EmptyState
        title="Search pixl-kb"
        description="Enter a query and press Enter or click Search."
      />
    );
  }
  if (state.kind === "loading") {
    return (
      <div className="space-y-3">
        {[...Array(3)].map((_, i) => (
          <div key={i} className="border border-line bg-panel p-4 space-y-2">
            <Skeleton className="h-4 w-2/5" />
            <Skeleton className="h-3 w-full" />
            <Skeleton className="h-3 w-3/4" />
          </div>
        ))}
      </div>
    );
  }
  if (state.kind === "error") {
    const detail = state.status
      ? `HTTP ${state.status}: ${state.message}`
      : state.message;
    return (
      <ErrorState
        title="KB search failed"
        description={`${detail} — ${KB_ERROR_HINT}`}
      />
    );
  }
  if (state.value.length === 0) {
    return (
      <EmptyState
        title="No results"
        description="Try a different query or check connection settings."
      />
    );
  }
  return (
    <div className="space-y-3">
      {state.value.map((item, i) => (
        <SearchResultCard key={`${item.source}-${i}`} item={item} />
      ))}
    </div>
  );
}

function SearchResultCard({ item }: { item: SearchResult }) {
  const snippet =
    item.content.length > 240
      ? `${item.content.slice(0, 240).trimEnd()}…`
      : item.content;

  return (
    <Card className="transition-colors hover:bg-overlay">
      <CardHeader className="pb-2">
        <CardTitle>{item.title}</CardTitle>
      </CardHeader>
      <CardContent>
        <p className="text-sm text-fg-2 leading-relaxed">{snippet}</p>
      </CardContent>
      <CardFooter className="gap-3">
        <span className="font-mono text-xs text-fg-3 truncate flex-1">{item.source}</span>
        <span className="shrink-0 font-mono text-xs text-fg-muted">
          score {item.score.toFixed(3)}
        </span>
      </CardFooter>
    </Card>
  );
}

// ── Documents tab ──

interface DocGroup {
  source: string;
  title: string;
  chunks: SearchResult[];
  expanded: boolean;
}

function DocumentsTab() {
  const [state, setState] = useState<AsyncState<DocGroup[]>>({ kind: "idle" });
  const [groups, setGroups] = useState<DocGroup[]>([]);

  const load = useCallback(async () => {
    setState({ kind: "loading" });
    try {
      const raw = await kbCall<unknown>("pixl_search", { query: "overview" });
      const items: SearchResult[] = Array.isArray(raw)
        ? raw.filter(
            (x): x is SearchResult =>
              x !== null && typeof x === "object" && "title" in x && "content" in x,
          )
        : [];

      // Group by source
      const map = new Map<string, SearchResult[]>();
      for (const item of items) {
        const key = item.source;
        const existing = map.get(key) ?? [];
        existing.push(item);
        map.set(key, existing);
      }

      const grouped: DocGroup[] = Array.from(map.entries()).map(([source, chunks]) => ({
        source,
        title: chunks[0]?.title ?? source,
        chunks,
        expanded: false,
      }));

      setGroups(grouped);
      setState({ kind: "data", value: grouped });
    } catch (err) {
      if (err instanceof KbError) {
        setState({ kind: "error", message: err.message, status: err.status });
      } else {
        setState({ kind: "error", message: err instanceof Error ? err.message : "Network error" });
      }
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  function toggleExpand(source: string) {
    setGroups((prev) =>
      prev.map((g) =>
        g.source === source ? { ...g, expanded: !g.expanded } : g,
      ),
    );
  }

  if (state.kind === "loading") {
    return (
      <div className="space-y-3">
        {[...Array(4)].map((_, i) => (
          <div key={i} className="border border-line bg-panel p-4 space-y-2">
            <Skeleton className="h-4 w-1/3" />
            <Skeleton className="h-3 w-24" />
          </div>
        ))}
      </div>
    );
  }
  if (state.kind === "error") {
    const detail = state.status
      ? `HTTP ${state.status}: ${state.message}`
      : state.message;
    return (
      <ErrorState
        title="Could not load documents"
        description={`${detail} — ${KB_ERROR_HINT}`}
        onRetry={() => void load()}
      />
    );
  }
  if (state.kind === "data" && groups.length === 0) {
    return (
      <EmptyState
        title="No documents found"
        description="The knowledge base is empty or the connection is not configured."
      />
    );
  }
  if (state.kind === "idle") {
    return null;
  }

  return (
    <div className="space-y-2">
      {groups.map((group) => (
        <DocGroupCard
          key={group.source}
          group={group}
          onToggle={() => toggleExpand(group.source)}
        />
      ))}
    </div>
  );
}

function DocGroupCard({
  group,
  onToggle,
}: {
  group: DocGroup;
  onToggle: () => void;
}) {
  return (
    <Card>
      <button
        type="button"
        onClick={onToggle}
        className="w-full text-left p-4 flex items-center justify-between hover:bg-overlay transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-teal-500"
        aria-expanded={group.expanded}
      >
        <div className="min-w-0">
          <p className="text-sm font-medium text-fg truncate">{group.title}</p>
          <p className="mt-0.5 font-mono text-xs text-fg-3 truncate">{group.source}</p>
        </div>
        <div className="ml-4 shrink-0 flex items-center gap-2">
          <Badge variant="default">{group.chunks.length} chunk{group.chunks.length !== 1 ? "s" : ""}</Badge>
          <span className="text-fg-muted text-xs">{group.expanded ? "▲" : "▼"}</span>
        </div>
      </button>
      {group.expanded && (
        <div>
          <Separator />
          <div className="p-4 space-y-3">
            {group.chunks.map((chunk, i) => (
              <div key={i} className="space-y-1">
                {i > 0 && <Separator className="my-3" />}
                <p className="text-xs font-medium text-fg-3">Chunk {i + 1}</p>
                <p className="text-sm text-fg-2 leading-relaxed">
                  {chunk.content.length > 300
                    ? `${chunk.content.slice(0, 300).trimEnd()}…`
                    : chunk.content}
                </p>
                <p className="font-mono text-xs text-fg-muted">
                  score {chunk.score.toFixed(3)}
                </p>
              </div>
            ))}
          </div>
        </div>
      )}
    </Card>
  );
}

// ── Graph tab ──

interface GraphEntity {
  name: string;
  type?: string;
  related?: string[];
  description?: string;
}

function GraphTab() {
  const [query, setQuery] = useState("overview");
  const [state, setState] = useState<AsyncState<GraphEntity[]>>({ kind: "idle" });

  const search = useCallback(async (q: string) => {
    const trimmed = q.trim();
    if (!trimmed) return;
    setState({ kind: "loading" });
    try {
      const raw = await kbCall<unknown>("pixl_kg_query", { query: trimmed });
      let entities: GraphEntity[] = [];
      if (Array.isArray(raw)) {
        entities = raw.filter(
          (x): x is GraphEntity =>
            x !== null && typeof x === "object" && "name" in x,
        );
      } else if (raw && typeof raw === "object") {
        // Some responses may wrap in { entities: [...] }
        const asRecord = raw as Record<string, unknown>;
        if (Array.isArray(asRecord.entities)) {
          entities = (asRecord.entities as unknown[]).filter(
            (x): x is GraphEntity =>
              x !== null && typeof x === "object" && "name" in (x as object),
          );
        }
      }
      setState({ kind: "data", value: entities });
    } catch (err) {
      if (err instanceof KbError) {
        setState({ kind: "error", message: err.message, status: err.status });
      } else {
        setState({ kind: "error", message: err instanceof Error ? err.message : "Network error" });
      }
    }
  }, []);

  function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
    if (e.key === "Enter") void search(query);
  }

  return (
    <>
      <div className="flex items-center gap-2">
        <Input
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Query knowledge graph…"
          className="flex-1"
          aria-label="Knowledge graph query"
        />
        <Button
          variant="outline"
          size="md"
          onClick={() => void search(query)}
          disabled={state.kind === "loading"}
        >
          {state.kind === "loading" ? "Querying…" : "Query"}
        </Button>
      </div>

      <div className="mt-4">
        <GraphResults state={state} />
      </div>
    </>
  );
}

function GraphResults({ state }: { state: AsyncState<GraphEntity[]> }) {
  if (state.kind === "idle") {
    return (
      <EmptyState
        title="Query the knowledge graph"
        description="Enter a concept and press Enter or click Query."
      />
    );
  }
  if (state.kind === "loading") {
    return (
      <div className="space-y-3">
        {[...Array(3)].map((_, i) => (
          <div key={i} className="border border-line bg-panel p-4 space-y-2">
            <Skeleton className="h-4 w-1/3" />
            <Skeleton className="h-3 w-1/2" />
          </div>
        ))}
      </div>
    );
  }
  if (state.kind === "error") {
    const detail = state.status
      ? `HTTP ${state.status}: ${state.message}`
      : state.message;
    return (
      <ErrorState
        title="Graph query failed"
        description={`${detail} — ${KB_ERROR_HINT}`}
      />
    );
  }
  if (state.value.length === 0) {
    return (
      <EmptyState
        title="No entities found"
        description="The graph returned no results for this query."
      />
    );
  }
  return (
    <div className="space-y-3">
      {state.value.map((entity, i) => (
        <GraphEntityCard key={`${entity.name}-${i}`} entity={entity} />
      ))}
    </div>
  );
}

function GraphEntityCard({ entity }: { entity: GraphEntity }) {
  return (
    <Card>
      <CardHeader className="pb-2">
        <div className="flex items-center gap-2">
          <CardTitle>{entity.name}</CardTitle>
          {entity.type && (
            <Badge variant="primary" className="shrink-0">{entity.type}</Badge>
          )}
        </div>
        {entity.description && (
          <CardDescription>{entity.description}</CardDescription>
        )}
      </CardHeader>
      {entity.related && entity.related.length > 0 && (
        <CardContent>
          <p className="text-xs text-fg-muted mb-1.5">Related</p>
          <div className="flex flex-wrap gap-1.5">
            {entity.related.map((rel) => (
              <Badge key={rel} variant="outline">{rel}</Badge>
            ))}
          </div>
        </CardContent>
      )}
    </Card>
  );
}

// ── Page ──

export default function Knowledge() {
  return (
    <div className="space-y-4">
      <div>
        <h2 className="text-sm font-medium text-fg">Knowledge</h2>
        <p className="mt-0.5 text-sm text-fg-3">
          Search and explore the pixl-kb knowledge base.
        </p>
      </div>
      <Separator />
      <Tabs defaultValue="search">
        <TabsList>
          <TabsTrigger value="search">Search</TabsTrigger>
          <TabsTrigger value="documents">Documents</TabsTrigger>
          <TabsTrigger value="graph">Graph</TabsTrigger>
        </TabsList>

        <TabsContent value="search">
          <SearchTab />
        </TabsContent>

        <TabsContent value="documents">
          <DocumentsTab />
        </TabsContent>

        <TabsContent value="graph">
          <GraphTab />
        </TabsContent>
      </Tabs>
    </div>
  );
}
