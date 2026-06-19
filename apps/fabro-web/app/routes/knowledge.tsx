import { useCallback, useEffect, useState } from "react";
import { Link } from "react-router";

import { EmptyState, ErrorState } from "../components/state";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "../components/ui/tabs";
import { Card, CardHeader, CardTitle, CardContent, CardFooter } from "../components/ui/card";
import { Input } from "../components/ui/input";
import { Button } from "../components/ui/button";
import { Badge } from "../components/ui/badge";
import { Skeleton } from "../components/ui/skeleton";
import { Separator } from "../components/ui/separator";
import { kbCall, KbError } from "../lib/kb";

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

const KB_ERROR_HINT = "check connection settings or set a pixl-kb token";

// ── Connection settings link ──

function ConnectionLink() {
  return (
    <Link
      to="/settings/knowledge"
      className="text-xs text-fg-muted hover:text-teal-500 transition-colors"
    >
      Connection settings →
    </Link>
  );
}

// ── Search tab ──

function SearchTab() {
  const [query, setQuery] = useState("overview");
  const [state, setState] = useState<AsyncState<SearchResult[]>>({ kind: "idle" });

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
      </div>
      <div className="mt-2 flex justify-end">
        <ConnectionLink />
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

function DocumentsTab({ active }: { active: boolean }) {
  const [state, setState] = useState<AsyncState<DocGroup[]>>({ kind: "idle" });
  const [groups, setGroups] = useState<DocGroup[]>([]);
  const [loaded, setLoaded] = useState(false);

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

  // Lazy-load: only trigger when the tab becomes active, and only once
  useEffect(() => {
    if (active && !loaded) {
      setLoaded(true);
      void load();
    }
  }, [active, loaded, load]);

  function toggleExpand(source: string) {
    setGroups((prev) =>
      prev.map((g) =>
        g.source === source ? { ...g, expanded: !g.expanded } : g,
      ),
    );
  }

  if (state.kind === "loading" || (state.kind === "idle" && loaded)) {
    // Show skeleton while loading (loaded=true means load() was triggered, state transitions
    // to "loading" synchronously, so the skeleton renders before the first await)
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
  if (state.kind === "idle") {
    // Tab not yet activated — render nothing (avoids loading on mount)
    return null;
  }
  if (state.kind === "error") {
    const detail = state.status
      ? `HTTP ${state.status}: ${state.message}`
      : state.message;
    return (
      <ErrorState
        title="Could not load documents"
        description={`${detail} — ${KB_ERROR_HINT}`}
        onRetry={() => {
          setLoaded(false);
        }}
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

interface GraphRelation {
  relation_type?: string;
  related_entity?: string;
  entity?: string;
  type?: string;
  [key: string]: unknown;
}

interface GraphResult {
  relations?: GraphRelation[];
  entities?: GraphRelation[];
  [key: string]: unknown;
}

function GraphTab() {
  const [entityName, setEntityName] = useState("");
  const [state, setState] = useState<AsyncState<GraphResult>>({ kind: "idle" });

  const query = useCallback(async (name: string) => {
    const trimmed = name.trim();
    if (!trimmed) return;
    setState({ kind: "loading" });
    try {
      const raw = await kbCall<unknown>("pixl_kg_query", {
        entity_name: trimmed,
        direction: "both",
      });
      // Accept whatever shape the server returns
      const result: GraphResult =
        raw !== null && typeof raw === "object" ? (raw as GraphResult) : {};
      setState({ kind: "data", value: result });
    } catch (err) {
      if (err instanceof KbError) {
        setState({ kind: "error", message: err.message, status: err.status });
      } else {
        setState({ kind: "error", message: err instanceof Error ? err.message : "Network error" });
      }
    }
  }, []);

  function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
    if (e.key === "Enter") void query(entityName);
  }

  return (
    <>
      <div className="flex items-center gap-2">
        <Input
          value={entityName}
          onChange={(e) => setEntityName(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="e.g. a person, project, or concept"
          className="flex-1"
          aria-label="Entity name to query"
        />
        <Button
          variant="outline"
          size="md"
          onClick={() => void query(entityName)}
          disabled={state.kind === "loading" || !entityName.trim()}
        >
          {state.kind === "loading" ? "Querying…" : "Query"}
        </Button>
      </div>
      <div className="mt-2 flex justify-end">
        <ConnectionLink />
      </div>

      <div className="mt-4">
        <GraphResults state={state} />
      </div>
    </>
  );
}

function GraphResults({ state }: { state: AsyncState<GraphResult> }) {
  if (state.kind === "idle") {
    return (
      <EmptyState
        title="Explore the knowledge graph"
        description="Enter an entity name to explore its relations in the knowledge graph."
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

  // Render relations from the result — defensive about shape
  const result = state.value;
  const items: GraphRelation[] = Array.isArray(result.relations)
    ? result.relations
    : Array.isArray(result.entities)
      ? result.entities
      : Array.isArray(result)
        ? (result as unknown as GraphRelation[])
        : [];

  if (items.length === 0) {
    // Try to render the raw object if items is empty but result has content
    const hasContent = Object.keys(result).length > 0;
    if (hasContent) {
      return (
        <div className="border border-line bg-panel p-4">
          <p className="text-xs text-fg-muted mb-2">Raw graph result</p>
          <pre className="text-xs text-fg-2 overflow-auto whitespace-pre-wrap">
            {JSON.stringify(result, null, 2)}
          </pre>
        </div>
      );
    }
    return (
      <EmptyState
        title="No relations found"
        description="The graph returned no results for this entity."
      />
    );
  }

  return (
    <div className="space-y-3">
      {items.map((rel, i) => (
        <GraphRelationCard key={i} relation={rel} />
      ))}
    </div>
  );
}

function GraphRelationCard({ relation }: { relation: GraphRelation }) {
  const relationType = relation.relation_type ?? relation.type ?? "relation";
  const relatedEntity = relation.related_entity ?? relation.entity;

  return (
    <Card>
      <CardHeader className="pb-2">
        <div className="flex items-center gap-2">
          <CardTitle>{relatedEntity ?? "Unknown entity"}</CardTitle>
          <Badge variant="primary" className="shrink-0">{relationType}</Badge>
        </div>
      </CardHeader>
    </Card>
  );
}

// ── Page ──

export default function Knowledge() {
  const [activeTab, setActiveTab] = useState("search");

  return (
    <div className="space-y-4">
      <div>
        <h2 className="text-sm font-medium text-fg">Knowledge</h2>
        <p className="mt-0.5 text-sm text-fg-3">
          Search and explore the pixl-kb knowledge base.
        </p>
      </div>
      <Separator />
      <Tabs defaultValue="search" onValueChange={setActiveTab}>
        <TabsList>
          <TabsTrigger value="search">Search</TabsTrigger>
          <TabsTrigger value="documents">Documents</TabsTrigger>
          <TabsTrigger value="graph">Graph</TabsTrigger>
        </TabsList>

        <TabsContent value="search">
          <SearchTab />
        </TabsContent>

        <TabsContent value="documents">
          <DocumentsTab active={activeTab === "documents"} />
        </TabsContent>

        <TabsContent value="graph">
          <GraphTab />
        </TabsContent>
      </Tabs>
    </div>
  );
}
