use std::path::PathBuf;

#[allow(
    unused_imports,
    reason = "Feature-gated branches consume these imports when optional backends are enabled."
)]
use anyhow::{Context, Result, bail};
use fabro_types::{RunId, RunSandbox, SandboxProvider};

use crate::SandboxEventCallback;
#[cfg(feature = "daytona")]
use crate::daytona::DaytonaSandbox;
#[cfg(feature = "docker")]
use crate::docker::DockerSandbox;
use crate::local::LocalSandbox;

/// Reconnect to a sandbox from a saved record.
///
/// `daytona_api_key` is forwarded to the Daytona SDK when the provider is
/// `"daytona"`. Pass `None` to fall back to the `DAYTONA_API_KEY` env var.
#[allow(
    clippy::unused_async,
    unused_variables,
    reason = "Feature-gated sandbox backends leave some parameters unused on partial builds."
)]
pub async fn reconnect(
    record: &RunSandbox,
    daytona_api_key: Option<String>,
) -> Result<Box<dyn crate::Sandbox>> {
    reconnect_for_run(record, daytona_api_key, None).await
}

#[allow(
    unused_variables,
    reason = "Feature-gated sandbox backends leave parameters unused on partial builds."
)]
pub async fn reconnect_for_run(
    record: &RunSandbox,
    daytona_api_key: Option<String>,
    run_id: Option<RunId>,
) -> Result<Box<dyn crate::Sandbox>> {
    reconnect_for_run_with_callback(record, daytona_api_key, run_id, None).await
}

#[allow(
    unused_variables,
    reason = "Feature-gated sandbox backends leave parameters unused on partial builds."
)]
pub async fn reconnect_for_run_with_callback(
    record: &RunSandbox,
    daytona_api_key: Option<String>,
    run_id: Option<RunId>,
    event_callback: Option<SandboxEventCallback>,
) -> Result<Box<dyn crate::Sandbox>> {
    match record.provider {
        SandboxProvider::Local => {
            let mut sandbox = LocalSandbox::new(PathBuf::from(&record.working_directory));
            if let Some(callback) = event_callback {
                sandbox.set_event_callback(callback);
            }
            Ok(Box::new(sandbox))
        }
        #[cfg(feature = "docker")]
        SandboxProvider::Docker => {
            let repo_cloned = record
                .repo_cloned
                .context("Docker run sandbox missing repo_cloned metadata")?;
            let mut sandbox = DockerSandbox::reconnect(
                &record.id,
                repo_cloned,
                record.clone_origin_url.clone(),
                record.clone_branch.clone(),
                run_id,
            )
            .await
            .context("Failed to reconnect Docker sandbox")?;
            if let Some(callback) = event_callback {
                sandbox.set_event_callback(callback);
            }
            Ok(Box::new(sandbox))
        }
        #[cfg(feature = "daytona")]
        SandboxProvider::Daytona => {
            let repo_cloned = record
                .repo_cloned
                .context("Daytona run sandbox missing repo_cloned metadata")?;

            let mut sandbox = DaytonaSandbox::reconnect(
                &record.id,
                daytona_api_key,
                repo_cloned,
                record.clone_origin_url.clone(),
                record.clone_branch.clone(),
            )
            .await
            .map_err(anyhow::Error::new)?;
            if let Some(callback) = event_callback {
                sandbox.set_event_callback(callback);
            }
            Ok(Box::new(sandbox))
        }
    }
}
