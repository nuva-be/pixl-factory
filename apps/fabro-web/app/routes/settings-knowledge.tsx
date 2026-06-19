import { useCallback, useState } from "react";
import { getKbConfig, kbCall, KbError, setKbConfig } from "../lib/kb";
import { Button } from "../components/ui/button";
import { Input } from "../components/ui/input";
import { Separator } from "../components/ui/separator";

export function meta() {
  return [{ title: "Knowledge Base — pixl-factory" }];
}

type TestState =
  | { kind: "idle" }
  | { kind: "loading" }
  | { kind: "ok"; workspaces: unknown }
  | { kind: "error"; message: string };

export default function SettingsKnowledge() {
  const cfg = getKbConfig();
  const [endpoint, setEndpointState] = useState(cfg.endpoint);
  const [token, setTokenState] = useState(cfg.token);
  const [workspace, setWorkspaceState] = useState(cfg.workspace);
  const [saved, setSaved] = useState(false);
  const [testState, setTestState] = useState<TestState>({ kind: "idle" });

  function handleEndpointChange(v: string) {
    setEndpointState(v);
    setSaved(false);
    setKbConfig({ endpoint: v });
  }

  function handleTokenChange(v: string) {
    setTokenState(v);
    setSaved(false);
    setKbConfig({ token: v });
  }

  function handleWorkspaceChange(v: string) {
    setWorkspaceState(v);
    setSaved(false);
    setKbConfig({ workspace: v });
  }

  function handleSave() {
    setKbConfig({ endpoint, token, workspace });
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  }

  const handleTest = useCallback(async () => {
    setTestState({ kind: "loading" });
    // Persist current values before testing
    setKbConfig({ endpoint, token, workspace });
    try {
      const result = await kbCall("pixl_list_workspaces", {});
      setTestState({ kind: "ok", workspaces: result });
    } catch (err) {
      if (err instanceof KbError) {
        setTestState({ kind: "error", message: `HTTP ${err.status ?? "?"}: ${err.message}` });
      } else {
        setTestState({
          kind: "error",
          message: err instanceof Error ? err.message : "Network error",
        });
      }
    }
  }, [endpoint, token, workspace]);

  return (
    <div className="space-y-6">
      <p className="max-w-[64ch] text-sm/6 text-fg-3 text-pretty">
        Connection settings for pixl-kb. Used by the Knowledge section and the
        run Memory tab to reach the pixl-kb MCP endpoint. Settings are saved in{" "}
        <code className="font-mono text-fg-2">localStorage</code>.
      </p>

      <Separator />

      <section className="space-y-4 max-w-lg">
        <div className="space-y-1.5">
          <label className="text-xs font-medium text-fg-3 uppercase tracking-wider" htmlFor="kb-endpoint">
            Endpoint
          </label>
          <p className="text-xs text-fg-muted">
            The pixl-kb MCP HTTP endpoint. Default:{" "}
            <code className="font-mono">http://localhost:8421/api/mcp/call</code>
          </p>
          <Input
            id="kb-endpoint"
            value={endpoint}
            onChange={(e) => handleEndpointChange(e.target.value)}
            placeholder="http://localhost:8421/api/mcp/call"
            className="font-mono text-xs"
            aria-label="pixl-kb MCP endpoint"
          />
        </div>

        <div className="space-y-1.5">
          <label className="text-xs font-medium text-fg-3 uppercase tracking-wider" htmlFor="kb-token">
            Bearer token
          </label>
          <p className="text-xs text-fg-muted">
            Optional authentication token for the pixl-kb API.
          </p>
          <Input
            id="kb-token"
            type="password"
            value={token}
            onChange={(e) => handleTokenChange(e.target.value)}
            placeholder="sk-…"
            className="font-mono text-xs"
            aria-label="pixl-kb bearer token"
          />
        </div>

        <div className="space-y-1.5">
          <label className="text-xs font-medium text-fg-3 uppercase tracking-wider" htmlFor="kb-workspace">
            Workspace ID
          </label>
          <p className="text-xs text-fg-muted">
            The pixl-kb workspace to query. Default:{" "}
            <code className="font-mono">42e3f37a-bfe2-41e2-9ea2-e05b24586b46</code>
          </p>
          <Input
            id="kb-workspace"
            value={workspace}
            onChange={(e) => handleWorkspaceChange(e.target.value)}
            placeholder="42e3f37a-bfe2-41e2-9ea2-e05b24586b46"
            className="font-mono text-xs"
            aria-label="pixl-kb workspace ID"
          />
        </div>

        <div className="flex items-center gap-3 pt-2">
          <Button variant="primary" size="sm" onClick={handleSave}>
            {saved ? "Saved" : "Save"}
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={() => void handleTest()}
            disabled={testState.kind === "loading"}
          >
            {testState.kind === "loading" ? "Testing…" : "Test connection"}
          </Button>
        </div>

        {testState.kind === "ok" && (
          <p className="text-xs text-teal-500">
            Connection successful.
          </p>
        )}
        {testState.kind === "error" && (
          <p className="text-xs text-coral-400">
            {testState.message}
          </p>
        )}
      </section>
    </div>
  );
}
