// ── KB configuration keys ──

const LS_ENDPOINT = "pixl.kb.endpoint";
const LS_TOKEN = "pixl.kb.token";
const LS_WORKSPACE = "pixl.kb.workspace";

const DEFAULT_ENDPOINT = "http://localhost:8421/api/mcp/call";
const DEFAULT_WORKSPACE = "42e3f37a-bfe2-41e2-9ea2-e05b24586b46";

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

export interface KbConfig {
  endpoint: string;
  token: string;
  workspace: string;
}

export function getKbConfig(): KbConfig {
  return {
    endpoint: readLS(LS_ENDPOINT, DEFAULT_ENDPOINT),
    token: readLS(LS_TOKEN, ""),
    workspace: readLS(LS_WORKSPACE, DEFAULT_WORKSPACE),
  };
}

export function setKbConfig(config: Partial<KbConfig>): void {
  if (config.endpoint !== undefined) writeLS(LS_ENDPOINT, config.endpoint);
  if (config.token !== undefined) writeLS(LS_TOKEN, config.token);
  if (config.workspace !== undefined) writeLS(LS_WORKSPACE, config.workspace);
}

// ── Typed error ──

export class KbError extends Error {
  constructor(
    message: string,
    public readonly status: number | undefined,
  ) {
    super(message);
    this.name = "KbError";
  }
}

// ── Core caller ──

/**
 * Call a pixl-kb MCP tool.
 *
 * @param tool - MCP tool name (e.g. "pixl_search")
 * @param args - Tool arguments (workspace_id is injected automatically)
 * @returns Parsed JSON from the first `text` content entry
 */
export async function kbCall<T = unknown>(
  tool: string,
  args: Record<string, unknown>,
): Promise<T> {
  const { endpoint, token, workspace } = getKbConfig();

  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    "X-Workspace-Id": workspace,
  };
  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }

  let res: Response;
  try {
    res = await fetch(endpoint, {
      method: "POST",
      headers,
      body: JSON.stringify({
        name: tool,
        arguments: { ...args, workspace_id: workspace },
      }),
    });
  } catch (err) {
    throw new KbError(
      err instanceof Error ? err.message : "Network error",
      undefined,
    );
  }

  if (!res.ok) {
    const text = await res.text().catch(() => "");
    throw new KbError(
      text || res.statusText || `HTTP ${res.status}`,
      res.status,
    );
  }

  const json = (await res.json()) as {
    content?: { type: string; text: string }[];
  };

  const textEntry = json.content?.find((c) => c.type === "text");
  if (!textEntry) {
    return [] as unknown as T;
  }

  try {
    return JSON.parse(textEntry.text) as T;
  } catch {
    throw new KbError(
      `Could not parse pixl-kb response: ${textEntry.text.slice(0, 120)}`,
      undefined,
    );
  }
}
