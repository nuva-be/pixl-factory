import { useState } from "react";
import { useNavigate } from "react-router";
import {
  ExclamationTriangleIcon,
  ClipboardDocumentIcon,
  ClipboardDocumentCheckIcon,
  ChevronDownIcon,
  ChevronUpIcon,
} from "@heroicons/react/24/outline";

import { Badge } from "../components/ui/badge";
import { Button } from "../components/ui/button";
import { PIXL_TEMPLATES, type WorkflowTemplate } from "../lib/templates";

export const handle = { wide: false };

export function meta({}: any) {
  return [{ title: "Templates — pixl-factory" }];
}

// ---------------------------------------------------------------------------
// Run creation
// ---------------------------------------------------------------------------

interface RunManifest {
  version: 1;
  cwd: string;
  title: string;
  target: {
    identifier: string;
    path: string;
  };
  workflows: {
    [path: string]: {
      source: string;
    };
  };
}

function buildManifest(template: WorkflowTemplate): RunManifest {
  const workflowPath = `.fabro/workflows/${template.id}/workflow.fabro`;
  return {
    version: 1,
    cwd: "/tmp/fabro-templates",
    title: template.name,
    target: {
      identifier: template.id,
      path: workflowPath,
    },
    workflows: {
      [workflowPath]: {
        source: template.dot,
      },
    },
  };
}

async function readErrorDetail(response: Response): Promise<string | null> {
  try {
    const body = (await response.clone().json()) as {
      errors?: { detail?: string; title?: string }[];
    };
    const first = body.errors?.[0];
    return first?.detail ?? first?.title ?? null;
  } catch {
    return null;
  }
}

// ---------------------------------------------------------------------------
// Backend badge
// ---------------------------------------------------------------------------

function BackendBadge({ backend }: { backend: string }) {
  const label = backend.toUpperCase();
  if (backend === "kb") {
    return (
      <Badge variant="outline" className="border-teal-500/40 text-teal-500 font-mono text-[10px]">
        {label}
      </Badge>
    );
  }
  if (backend === "acp") {
    return (
      <Badge variant="outline" className="border-fg-muted/40 text-fg-muted font-mono text-[10px]">
        {label}
      </Badge>
    );
  }
  return (
    <Badge variant="outline" className="font-mono text-[10px]">
      {label}
    </Badge>
  );
}

// ---------------------------------------------------------------------------
// Copy-command button
// ---------------------------------------------------------------------------

function CopyCommandButton({ command }: { command: string }) {
  const [copied, setCopied] = useState(false);

  function handleCopy() {
    navigator.clipboard.writeText(command).catch(() => {});
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  }

  return (
    <button
      type="button"
      onClick={handleCopy}
      title="Copy command"
      aria-label="Copy run command"
      className="flex shrink-0 items-center justify-center rounded p-1 text-fg-muted transition-colors hover:bg-overlay hover:text-fg"
    >
      {copied ? (
        <ClipboardDocumentCheckIcon className="size-4 text-teal-500" aria-hidden="true" />
      ) : (
        <ClipboardDocumentIcon className="size-4" aria-hidden="true" />
      )}
    </button>
  );
}

// ---------------------------------------------------------------------------
// Template card
// ---------------------------------------------------------------------------

interface TemplateCardProps {
  template: WorkflowTemplate;
}

function TemplateCard({ template }: TemplateCardProps) {
  const navigate = useNavigate();
  const [running, setRunning] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showGraph, setShowGraph] = useState(false);

  const cliCommand = `fabro run examples/pixl/${template.tomlFile} --auto-approve`;

  async function handleRun() {
    setRunning(true);
    setError(null);
    try {
      const manifest = buildManifest(template);
      const response = await fetch("/api/v1/runs", {
        method: "POST",
        credentials: "same-origin",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(manifest),
      });
      if (!response.ok) {
        const detail = await readErrorDetail(response);
        throw new Error(detail ?? `${response.status} ${response.statusText}`);
      }
      const body = (await response.json()) as { id?: string };
      if (!body.id) {
        throw new Error("Server did not return a run id.");
      }
      // POST /runs only creates the run (status `submitted`); it must be
      // started explicitly or it never executes. A freshly-created run can be
      // briefly un-startable (400), so retry once; 409 means it's already
      // running, which is success for our purposes.
      let started = false;
      for (let attempt = 0; attempt < 2 && !started; attempt++) {
        const startRes = await fetch(`/api/v1/runs/${body.id}/start`, {
          method: "POST",
          credentials: "same-origin",
        });
        if (startRes.ok || startRes.status === 409) {
          started = true;
        } else if (attempt === 0 && startRes.status === 400) {
          await new Promise((r) => setTimeout(r, 700));
        } else {
          const detail = await readErrorDetail(startRes);
          throw new Error(detail ?? `Failed to start run: ${startRes.status} ${startRes.statusText}`);
        }
      }
      navigate(`/runs/${body.id}`);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
      setRunning(false);
    }
  }

  return (
    <article className="flex flex-col border border-line bg-panel">
      {/* Card header */}
      <div className="flex flex-col gap-3 p-5">
        <div className="flex items-start justify-between gap-3">
          <h2 className="text-sm font-semibold text-fg">{template.name}</h2>
          <div className="flex shrink-0 flex-wrap gap-1.5">
            {template.backends.map((b) => (
              <BackendBadge key={b} backend={b} />
            ))}
          </div>
        </div>
        <p className="text-sm text-fg-3 leading-relaxed">{template.description}</p>
      </div>

      {/* Error state */}
      {error && (
        <div className="mx-5 mb-4 flex items-start gap-2 border border-coral/30 bg-coral/10 p-3 text-xs text-coral">
          <ExclamationTriangleIcon className="mt-0.5 size-4 shrink-0" aria-hidden="true" />
          <div>
            <div className="mb-0.5 font-semibold">Run failed</div>
            <div className="break-words">{error}</div>
            <div className="mt-2 border-t border-coral/20 pt-2 text-fg-muted">
              Run locally instead:
              <div className="mt-1 flex items-center gap-1 rounded bg-page px-2 py-1 font-mono">
                <span className="min-w-0 flex-1 truncate">{cliCommand}</span>
                <CopyCommandButton command={cliCommand} />
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Graph toggle */}
      <div className="border-t border-line">
        <button
          type="button"
          onClick={() => setShowGraph((v) => !v)}
          aria-expanded={showGraph}
          className="flex w-full items-center gap-2 px-5 py-2.5 text-left text-xs text-fg-muted transition-colors hover:bg-overlay hover:text-fg"
        >
          {showGraph ? (
            <ChevronUpIcon className="size-3.5 shrink-0" aria-hidden="true" />
          ) : (
            <ChevronDownIcon className="size-3.5 shrink-0" aria-hidden="true" />
          )}
          <span>{showGraph ? "Hide graph" : "View graph"}</span>
        </button>

        {showGraph && (
          <div className="border-t border-line bg-page px-5 py-4">
            <pre
              className="overflow-x-auto whitespace-pre font-mono text-[11px] leading-relaxed text-fg-3"
              aria-label={`${template.name} workflow graph source`}
            >
              {template.dot}
            </pre>
          </div>
        )}
      </div>

      {/* Card footer / actions */}
      <div className="mt-auto flex items-center justify-between border-t border-line px-5 py-3">
        <div className="flex items-center gap-1.5 rounded border border-line bg-page px-2 py-1">
          <span className="font-mono text-[10px] text-fg-muted truncate max-w-[180px]">
            {cliCommand}
          </span>
          <CopyCommandButton command={cliCommand} />
        </div>

        <Button
          variant="primary"
          size="sm"
          onClick={handleRun}
          disabled={running}
          aria-label={`Run ${template.name}`}
        >
          {running ? "Launching…" : "Run"}
        </Button>
      </div>
    </article>
  );
}

// ---------------------------------------------------------------------------
// Page
// ---------------------------------------------------------------------------

export default function Templates() {
  return (
    <div>
      <div className="mb-6">
        <h1 className="text-xl font-semibold tracking-tight text-fg">Templates</h1>
        <p className="mt-1 text-sm text-fg-3">
          One-click pixl workflow examples. Click{" "}
          <span className="font-medium text-teal-500">Run</span> to launch in a local
          sandbox, or copy the CLI command to run from your terminal.
        </p>
      </div>

      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
        {PIXL_TEMPLATES.map((template) => (
          <TemplateCard key={template.id} template={template} />
        ))}
      </div>
    </div>
  );
}
