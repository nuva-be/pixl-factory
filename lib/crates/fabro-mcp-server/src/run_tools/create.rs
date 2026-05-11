use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use fabro_client::Client;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::{ToolError, ToolResult};
use super::{common, manifest};

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct FabroRunCreateParams {
    pub(crate) runs: Vec<CreateRunSpec>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct CreateRunSpec {
    pub(crate) workflow:         String,
    pub(crate) cwd:              Option<PathBuf>,
    pub(crate) run_id:           Option<String>,
    pub(crate) goal:             Option<String>,
    #[serde(default)]
    pub(crate) inputs:           HashMap<String, Value>,
    #[serde(default)]
    pub(crate) labels:           HashMap<String, String>,
    pub(crate) dry_run:          Option<bool>,
    pub(crate) auto_approve:     Option<bool>,
    pub(crate) model:            Option<String>,
    pub(crate) provider:         Option<String>,
    pub(crate) sandbox:          Option<String>,
    pub(crate) preserve_sandbox: Option<bool>,
    pub(crate) start:            Option<bool>,
}

#[derive(Debug)]
pub(crate) struct ValidatedCreateRuns {
    pub(crate) runs: Vec<CreateRunSpec>,
}

impl TryFrom<FabroRunCreateParams> for ValidatedCreateRuns {
    type Error = ToolError;

    fn try_from(params: FabroRunCreateParams) -> Result<Self, Self::Error> {
        common::validate_len("runs", params.runs.len(), 1, 50)?;
        for spec in &params.runs {
            for (key, value) in &spec.inputs {
                manifest::json_to_toml_value(key, value)?;
            }
        }
        Ok(Self { runs: params.runs })
    }
}

#[derive(Debug, Serialize, JsonSchema)]
pub(crate) struct CreateRunsResult {
    pub(crate) runs: Vec<CreatedRunResult>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub(crate) struct CreatedRunResult {
    pub(crate) run_id:   String,
    pub(crate) workflow: String,
    pub(crate) started:  bool,
    pub(crate) status:   String,
}

pub(crate) async fn create_runs(
    client: Arc<Client>,
    base_cwd: &Path,
    user_settings_path: &Path,
    params: ValidatedCreateRuns,
) -> ToolResult<CreateRunsResult> {
    let mut created = Vec::with_capacity(params.runs.len());
    for spec in params.runs {
        let cwd = spec.cwd.clone().unwrap_or_else(|| base_cwd.to_path_buf());
        let manifest = manifest::build_mcp_run_manifest(&spec, &cwd, user_settings_path)?;
        let run_id = client
            .create_run_from_manifest(manifest)
            .await
            .map_err(|err| ToolError::from_anyhow(&err))?;
        let started = spec.start.unwrap_or(true);
        if started {
            client
                .start_run(&run_id, false)
                .await
                .map_err(|err| ToolError::from_anyhow(&err))?;
        }
        let summary = client
            .retrieve_run(&run_id)
            .await
            .map_err(|err| ToolError::from_anyhow(&err))?;
        created.push(CreatedRunResult {
            run_id: summary.id.to_string(),
            workflow: spec.workflow,
            started,
            status: common::run_status_kind(summary.lifecycle.status).to_string(),
        });
    }
    Ok(CreateRunsResult { runs: created })
}

pub(crate) fn create_runs_text(result: &CreateRunsResult) -> String {
    let started = result.runs.iter().filter(|run| run.started).count();
    format!(
        "created {} Fabro run(s), started {started}",
        result.runs.len()
    )
}
