//! Native pixl-kb backend (`backend="kb"`).
//!
//! Calls the pixl-kb MCP gateway (`POST <endpoint>/api/mcp/call`) directly from
//! the engine — no LLM, no agent loop. A `backend="kb"` node is a deterministic,
//! token-free-of-LLM-cost graph step that recalls from or writes to pixl-kb and
//! returns the tool's text output as the node response (which downstream nodes
//! and routing can read).
//!
//! Node attributes:
//! - `kb.tool`      — MCP tool name (default `pixl_search`)
//! - `kb.endpoint`  — gateway URL (default `http://localhost:8421/api/mcp/call`)
//! - `kb.workspace` — workspace id (→ `X-Workspace-Id` header + `workspace_id` arg)
//! - `kb.args`      — JSON object of tool arguments; when absent, the node prompt
//!                    is sent as `{"query": <prompt>}`
//! - `kb.token`     — bearer token; falls back to `$PIXL_KB_TOKEN`
//!
//! The gateway returns `{"content": [{"type": "text", "text": "..."}]}`; the
//! backend joins the text parts into the node response.

use async_trait::async_trait;
use fabro_graphviz::graph::Node;
use fabro_types::StageTiming;
use serde_json::{Map, Value};

use super::super::agent::{CodergenBackend, CodergenResult, CodergenRunRequest};
use crate::error::Error;

const DEFAULT_ENDPOINT: &str = "http://localhost:8421/api/mcp/call";
const DEFAULT_TOOL: &str = "pixl_search";

/// Backend that turns a `backend="kb"` node into one pixl-kb MCP gateway call.
#[derive(Default)]
pub struct AgentKbBackend;

impl AgentKbBackend {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

/// Build the `arguments` object: parse `kb.args` JSON if present, else seed with
/// the prompt as `query`. The workspace id is always injected when known so a
/// node never silently writes to the wrong workspace.
fn build_arguments(node: &Node, prompt: &str, workspace: Option<&str>) -> Result<Value, Error> {
    let mut args: Map<String, Value> = match node.kb_args_attr() {
        Some(raw) => serde_json::from_str(raw).map_err(|err| {
            Error::Validation(format!("kb.args must be a JSON object: {err}"))
        })?,
        None => {
            let mut map = Map::new();
            map.insert("query".to_string(), Value::String(prompt.to_string()));
            map
        }
    };
    if let Some(ws) = workspace {
        args.entry("workspace_id".to_string())
            .or_insert_with(|| Value::String(ws.to_string()));
    }
    Ok(Value::Object(args))
}

/// Flatten the gateway's `{"content": [{"text": ...}]}` envelope into one string.
fn extract_text(body: &Value) -> String {
    if let Some(content) = body.get("content").and_then(Value::as_array) {
        let joined: Vec<&str> = content
            .iter()
            .filter_map(|c| c.get("text").and_then(Value::as_str))
            .collect();
        if !joined.is_empty() {
            return joined.join("\n");
        }
    }
    // Fall back to the raw JSON so the node never returns an empty response.
    body.to_string()
}

#[async_trait]
impl CodergenBackend for AgentKbBackend {
    async fn run(&self, request: CodergenRunRequest<'_>) -> Result<CodergenResult, Error> {
        let node = request.node;
        let tool = node.kb_tool_attr().unwrap_or(DEFAULT_TOOL);
        let endpoint = node.kb_endpoint_attr().unwrap_or(DEFAULT_ENDPOINT);
        let workspace = node.kb_workspace_attr();
        let token = node
            .kb_token_attr()
            .map(String::from)
            .or_else(|| std::env::var("PIXL_KB_TOKEN").ok());

        let arguments = build_arguments(node, request.prompt, workspace)?;
        let payload = serde_json::json!({ "name": tool, "arguments": arguments });

        let client = fabro_http::http_client()
            .map_err(|err| Error::handler(format!("kb: failed to build http client: {err}")))?;
        let mut req = client.post(endpoint).json(&payload);
        if let Some(ws) = workspace {
            req = req.header("X-Workspace-Id", ws);
        }
        if let Some(tok) = &token {
            req = req.header("Authorization", format!("Bearer {tok}"));
        }

        let resp = req
            .send()
            .await
            .map_err(|err| Error::handler(format!("kb: request to {endpoint} failed: {err}")))?;
        let status = resp.status();
        let raw = resp
            .text()
            .await
            .map_err(|err| Error::handler(format!("kb: failed to read response body: {err}")))?;
        if !status.is_success() {
            // 4xx (bad tool/args/auth) is terminal config; 5xx is transient.
            let msg = format!("kb: {tool} -> HTTP {status}: {}", raw.trim());
            return Err(if status.is_client_error() {
                Error::Validation(msg)
            } else {
                Error::handler(msg)
            });
        }

        let body: Value = serde_json::from_str(&raw)
            .map_err(|err| Error::handler(format!("kb: invalid JSON response: {err}")))?;
        let text = extract_text(&body);

        Ok(CodergenResult::Text {
            text,
            usage: None,
            files_touched: Vec::new(),
            last_file_touched: None,
            timing: StageTiming::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use fabro_graphviz::graph::{AttrValue, Node};

    use super::*;

    fn node_with(attrs: &[(&str, &str)]) -> Node {
        let mut node = Node::new("recall");
        for (k, v) in attrs {
            node.attrs
                .insert((*k).to_string(), AttrValue::String((*v).to_string()));
        }
        node
    }

    #[test]
    fn build_arguments_defaults_prompt_to_query() {
        let node = node_with(&[]);
        let args = build_arguments(&node, "overview", Some("ws-1")).unwrap();
        assert_eq!(args["query"], Value::String("overview".to_string()));
        assert_eq!(args["workspace_id"], Value::String("ws-1".to_string()));
    }

    #[test]
    fn build_arguments_parses_explicit_json_and_keeps_its_workspace() {
        let node = node_with(&[("kb.args", r#"{"query":"x","workspace_id":"explicit"}"#)]);
        let args = build_arguments(&node, "ignored", Some("ws-1")).unwrap();
        assert_eq!(args["query"], Value::String("x".to_string()));
        // explicit workspace_id is not overwritten by the node attr
        assert_eq!(args["workspace_id"], Value::String("explicit".to_string()));
    }

    #[test]
    fn build_arguments_rejects_malformed_json() {
        let node = node_with(&[("kb.args", "{not json")]);
        let err = build_arguments(&node, "p", None).unwrap_err();
        assert!(err.to_string().contains("kb.args must be a JSON object"));
    }

    #[test]
    fn extract_text_joins_content_parts() {
        let body = serde_json::json!({
            "content": [{"type": "text", "text": "a"}, {"type": "text", "text": "b"}]
        });
        assert_eq!(extract_text(&body), "a\nb");
    }

    #[test]
    fn extract_text_falls_back_to_raw_when_no_content() {
        let body = serde_json::json!({"unexpected": true});
        assert_eq!(extract_text(&body), body.to_string());
    }
}
