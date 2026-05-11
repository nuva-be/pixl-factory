use fabro_graphviz::graph::{AttrValue, Graph};

use crate::{Diagnostic, LintRule, Severity};

pub(super) fn rule() -> Box<dyn LintRule> {
    Box::new(Rule)
}

struct Rule;

const VALID_BACKENDS: &[&str] = &["api", "cli", "acp"];

impl LintRule for Rule {
    fn name(&self) -> &'static str {
        "backend_valid"
    }

    fn apply(&self, graph: &Graph) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for node in graph.nodes.values() {
            if let Some(backend) = node.attrs.get("backend").and_then(AttrValue::as_str) {
                if !VALID_BACKENDS.contains(&backend) {
                    diagnostics.push(Diagnostic {
                        rule:     self.name().to_string(),
                        severity: Severity::Error,
                        message:  format!(
                            "unsupported LLM backend \"{backend}\"; expected one of: api, cli, acp"
                        ),
                        node_id:  Some(node.id.clone()),
                        edge:     None,
                        fix:      Some("Use one of: api, cli, acp".to_string()),
                    });
                }
            }
        }
        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use fabro_graphviz::graph::{AttrValue, Node};

    use super::Rule;
    use crate::rules::test_support::minimal_graph;
    use crate::{LintRule, Severity};

    #[test]
    fn backend_valid_accepts_absent_api_cli_and_acp() {
        for backend in [None, Some("api"), Some("cli"), Some("acp")] {
            let mut graph = minimal_graph();
            let mut node = Node::new("work");
            if let Some(backend) = backend {
                node.attrs.insert(
                    "backend".to_string(),
                    AttrValue::String(backend.to_string()),
                );
            }
            graph.nodes.insert("work".to_string(), node);

            assert!(Rule.apply(&graph).is_empty(), "backend: {backend:?}");
        }
    }

    #[test]
    fn backend_valid_rejects_unknown_backend() {
        let mut graph = minimal_graph();
        let mut node = Node::new("work");
        node.attrs.insert(
            "backend".to_string(),
            AttrValue::String("codex".to_string()),
        );
        graph.nodes.insert("work".to_string(), node);

        let diagnostics = Rule.apply(&graph);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, Severity::Error);
        assert!(
            diagnostics[0]
                .message
                .contains("unsupported LLM backend \"codex\"; expected one of: api, cli, acp")
        );
    }
}
