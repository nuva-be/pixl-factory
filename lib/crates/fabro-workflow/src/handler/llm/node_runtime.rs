use std::sync::Arc;

use fabro_agent::Sandbox;
use tokio_util::sync::CancellationToken;

use crate::error::Error;

pub fn ensure_node_runtime_shell() -> String {
    "export PATH=\"$HOME/.local/bin:$PATH\" && \
     (node --version >/dev/null 2>&1 && npm --version >/dev/null 2>&1 && npx --version >/dev/null 2>&1 || \
      (mkdir -p ~/.local && curl -fsSL https://nodejs.org/dist/v22.14.0/node-v22.14.0-linux-x64.tar.gz | tar -xz --strip-components=1 -C ~/.local))"
        .to_string()
}

pub async fn ensure_node_runtime(
    sandbox: &Arc<dyn Sandbox>,
    cancel_token: &CancellationToken,
) -> Result<(), Error> {
    let command = ensure_node_runtime_shell();
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
        Ok(())
    } else {
        Err(Error::handler(format!(
            "Node runtime install exited with code {}",
            result.display_exit_code()
        )))
    }
}
