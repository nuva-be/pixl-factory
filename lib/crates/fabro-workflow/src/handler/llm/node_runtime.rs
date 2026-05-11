use std::sync::Arc;

use fabro_agent::Sandbox;
use fabro_static::EnvVars;
use tokio_util::sync::CancellationToken;

use crate::error::Error;

const NODE_RUNTIME_PATH_MARKER: &str = "__FABRO_NODE_RUNTIME_PATH=";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeRuntimeEnv {
    pub path: String,
}

pub fn ensure_node_runtime_shell() -> String {
    r#"export PATH="$HOME/.local/bin:$PATH" && \
if node --version >/dev/null 2>&1 && npm --version >/dev/null 2>&1 && npx --version >/dev/null 2>&1; then \
  true; \
else \
  os="$(uname -s)"; \
  if [ "$os" != "Linux" ]; then \
    echo "Node.js, npm, and npx are required for default ACP/CLI commands on $os" >&2; \
    exit 127; \
  fi; \
  arch="$(uname -m)"; \
  case "$arch" in \
    x86_64|amd64) node_arch="x64" ;; \
    aarch64|arm64) node_arch="arm64" ;; \
    *) echo "Unsupported Linux architecture for Node.js install: $arch" >&2; exit 127 ;; \
  esac; \
  mkdir -p "$HOME/.local" && \
  curl -fsSL "https://nodejs.org/dist/v22.14.0/node-v22.14.0-linux-${node_arch}.tar.gz" | tar -xz --strip-components=1 -C "$HOME/.local"; \
fi"#
        .to_string()
}

pub async fn ensure_node_runtime(
    sandbox: &Arc<dyn Sandbox>,
    cancel_token: &CancellationToken,
) -> Result<NodeRuntimeEnv, Error> {
    let command = format!(
        "{} && printf '\\n{}%s\\n' \"$PATH\"",
        ensure_node_runtime_shell(),
        NODE_RUNTIME_PATH_MARKER
    );
    let result = sandbox
        .exec_command(
            &command,
            180_000,
            None,
            None,
            Some(cancel_token.child_token()),
        )
        .await
        .map_err(|err| Error::handler_with_source("Failed to ensure Node runtime", &err))?;

    if result.is_success() {
        let path = parse_node_runtime_path(&result.stdout).ok_or_else(|| {
            Error::handler("Node runtime install did not report the sandbox PATH".to_string())
        })?;
        Ok(NodeRuntimeEnv { path })
    } else {
        Err(Error::handler(format!(
            "Node runtime install exited with code {}",
            result.display_exit_code()
        )))
    }
}

pub fn apply_node_runtime_env(
    launch_env: &mut std::collections::HashMap<String, String>,
    runtime_env: NodeRuntimeEnv,
) {
    match launch_env.get_mut(EnvVars::PATH) {
        Some(existing_path) if !existing_path.is_empty() => {
            *existing_path = format!("{}:{existing_path}", runtime_env.path);
        }
        _ => {
            launch_env.insert(EnvVars::PATH.to_string(), runtime_env.path);
        }
    }
}

fn parse_node_runtime_path(stdout: &str) -> Option<String> {
    stdout
        .lines()
        .rev()
        .find_map(|line| line.strip_prefix(NODE_RUNTIME_PATH_MARKER))
        .filter(|path| !path.is_empty())
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{NodeRuntimeEnv, apply_node_runtime_env, parse_node_runtime_path};

    #[test]
    fn parse_node_runtime_path_uses_last_reported_marker() {
        assert_eq!(
            parse_node_runtime_path(
                "download output\n__FABRO_NODE_RUNTIME_PATH=/old\n\
                 __FABRO_NODE_RUNTIME_PATH=/home/test/.local/bin:/usr/bin\n",
            ),
            Some("/home/test/.local/bin:/usr/bin".to_string())
        );
    }

    #[test]
    fn apply_node_runtime_env_preserves_existing_path_tail() {
        let mut env = HashMap::from([("PATH".to_string(), "/custom/bin".to_string())]);

        apply_node_runtime_env(&mut env, NodeRuntimeEnv {
            path: "/home/test/.local/bin:/usr/bin".to_string(),
        });

        assert_eq!(
            env.get("PATH").map(String::as_str),
            Some("/home/test/.local/bin:/usr/bin:/custom/bin")
        );
    }
}
