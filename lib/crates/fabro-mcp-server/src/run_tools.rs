#![allow(
    dead_code,
    reason = "The MCP server skeleton defines the full first-slice contract before each tool body is implemented."
)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use fabro_client::Client;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::task::yield_now;

#[derive(Debug)]
pub(crate) struct ToolError {
    message: String,
}

impl ToolError {
    pub(crate) fn message(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub(crate) fn from_anyhow(err: &anyhow::Error) -> Self {
        Self::message(format_tool_error(err))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.message
    }
}

pub(crate) type ToolResult<T> = Result<T, ToolError>;

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
        validate_len("runs", params.runs.len(), 1, 50)?;
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

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct FabroRunSearchParams {
    pub(crate) run_ids:        Option<Vec<String>>,
    pub(crate) workflow:       Option<String>,
    pub(crate) labels:         Option<HashMap<String, String>>,
    pub(crate) status:         Option<Vec<String>>,
    pub(crate) archived:       Option<bool>,
    pub(crate) created_after:  Option<String>,
    pub(crate) created_before: Option<String>,
    pub(crate) first:          Option<usize>,
    pub(crate) after:          Option<String>,
}

#[derive(Debug)]
pub(crate) struct ValidatedSearchRuns {
    pub(crate) raw: FabroRunSearchParams,
}

impl TryFrom<FabroRunSearchParams> for ValidatedSearchRuns {
    type Error = ToolError;

    fn try_from(params: FabroRunSearchParams) -> Result<Self, Self::Error> {
        if params.first.is_some_and(|first| first > 100) {
            return Err(ToolError::message("first must be <= 100"));
        }
        Ok(Self { raw: params })
    }
}

#[derive(Debug, Serialize, JsonSchema)]
pub(crate) struct SearchRunsResult {
    pub(crate) runs:        Vec<RunSummaryResult>,
    pub(crate) next_cursor: Option<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub(crate) struct RunSummaryResult {
    pub(crate) run_id:           String,
    pub(crate) workflow_name:    String,
    pub(crate) workflow_slug:    Option<String>,
    pub(crate) status:           String,
    pub(crate) archived:         bool,
    pub(crate) created_at:       String,
    pub(crate) started_at:       Option<String>,
    pub(crate) completed_at:     Option<String>,
    pub(crate) labels:           HashMap<String, String>,
    pub(crate) source_directory: Option<String>,
    pub(crate) repo_origin_url:  Option<String>,
    pub(crate) goal:             String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RunInteractAction {
    Get,
    Start,
    Message,
    Cancel,
    Archive,
    Unarchive,
    GetQuestions,
    Answer,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct FabroRunInteractParams {
    pub(crate) action:      RunInteractAction,
    pub(crate) run_id:      String,
    pub(crate) message:     Option<String>,
    pub(crate) interrupt:   Option<bool>,
    pub(crate) question_id: Option<String>,
    pub(crate) answer:      Option<Value>,
}

#[derive(Debug)]
pub(crate) struct ValidatedInteractRun {
    pub(crate) raw: FabroRunInteractParams,
}

impl TryFrom<FabroRunInteractParams> for ValidatedInteractRun {
    type Error = ToolError;

    fn try_from(params: FabroRunInteractParams) -> Result<Self, Self::Error> {
        if params.run_id.trim().is_empty() {
            return Err(ToolError::message("run_id is required"));
        }
        if matches!(params.action, RunInteractAction::Message)
            && params
                .message
                .as_deref()
                .is_none_or(|message| message.trim().is_empty())
        {
            return Err(ToolError::message("message is required for action message"));
        }
        if matches!(params.action, RunInteractAction::Answer) {
            if params.question_id.as_deref().is_none_or(str::is_empty) {
                return Err(ToolError::message(
                    "question_id is required for action answer",
                ));
            }
            if params.answer.is_none() {
                return Err(ToolError::message("answer is required for action answer"));
            }
        }
        Ok(Self { raw: params })
    }
}

#[derive(Debug, Serialize, JsonSchema)]
pub(crate) struct InteractRunResult {
    pub(crate) run_id: String,
    pub(crate) action: RunInteractAction,
    pub(crate) result: Value,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct FabroRunGatherParams {
    pub(crate) run_ids:               Vec<String>,
    pub(crate) timeout_seconds:       Option<u64>,
    pub(crate) poll_interval_seconds: Option<u64>,
}

#[derive(Debug)]
pub(crate) struct ValidatedGatherRuns {
    pub(crate) run_ids:               Vec<String>,
    pub(crate) timeout_seconds:       u64,
    pub(crate) poll_interval_seconds: u64,
}

impl TryFrom<FabroRunGatherParams> for ValidatedGatherRuns {
    type Error = ToolError;

    fn try_from(params: FabroRunGatherParams) -> Result<Self, Self::Error> {
        validate_len("run_ids", params.run_ids.len(), 1, 50)?;
        Ok(Self {
            run_ids:               params.run_ids,
            timeout_seconds:       params.timeout_seconds.unwrap_or(300).min(600),
            poll_interval_seconds: params.poll_interval_seconds.unwrap_or(15).max(5),
        })
    }
}

#[derive(Debug, Serialize, JsonSchema)]
pub(crate) struct GatherRunsResult {
    pub(crate) runs:            Vec<RunSummaryResult>,
    pub(crate) timed_out:       bool,
    pub(crate) elapsed_seconds: u64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RunEventsAction {
    List,
    Details,
    Search,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct FabroRunEventsParams {
    pub(crate) action:             RunEventsAction,
    pub(crate) run_id:             String,
    pub(crate) event_types:        Option<Vec<String>>,
    pub(crate) categories:         Option<Vec<String>>,
    pub(crate) direction:          Option<String>,
    pub(crate) created_after:      Option<String>,
    pub(crate) created_before:     Option<String>,
    pub(crate) first:              Option<usize>,
    pub(crate) after:              Option<u32>,
    pub(crate) event_ids:          Option<Vec<String>>,
    pub(crate) offset:             Option<usize>,
    pub(crate) limit:              Option<usize>,
    pub(crate) max_content_length: Option<usize>,
    pub(crate) query:              Option<String>,
}

#[derive(Debug)]
pub(crate) struct ValidatedRunEvents {
    pub(crate) raw: FabroRunEventsParams,
}

impl TryFrom<FabroRunEventsParams> for ValidatedRunEvents {
    type Error = ToolError;

    fn try_from(params: FabroRunEventsParams) -> Result<Self, Self::Error> {
        if params.run_id.trim().is_empty() {
            return Err(ToolError::message("run_id is required"));
        }
        let first = params.first.or(params.limit).unwrap_or(50);
        if first > 200 {
            return Err(ToolError::message("first must be <= 200"));
        }
        Ok(Self { raw: params })
    }
}

#[derive(Debug, Serialize, JsonSchema)]
pub(crate) struct RunEventsResult {
    pub(crate) run_id:      String,
    pub(crate) action:      RunEventsAction,
    pub(crate) events:      Vec<RunEventResult>,
    pub(crate) next_cursor: Option<u32>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub(crate) struct RunEventResult {
    pub(crate) event_id:  String,
    pub(crate) sequence:  u32,
    pub(crate) event:     Value,
    pub(crate) truncated: bool,
}

pub(crate) async fn create_runs(
    _client: Arc<Client>,
    _base_cwd: &Path,
    _params: ValidatedCreateRuns,
) -> ToolResult<CreateRunsResult> {
    yield_now().await;
    Err(ToolError::message(
        "fabro_run_create is not implemented yet",
    ))
}

pub(crate) async fn search_runs(
    _client: Arc<Client>,
    _params: ValidatedSearchRuns,
) -> ToolResult<SearchRunsResult> {
    yield_now().await;
    Err(ToolError::message(
        "fabro_run_search is not implemented yet",
    ))
}

pub(crate) async fn interact_run(
    _client: Arc<Client>,
    _params: ValidatedInteractRun,
) -> ToolResult<InteractRunResult> {
    yield_now().await;
    Err(ToolError::message(
        "fabro_run_interact is not implemented yet",
    ))
}

pub(crate) async fn gather_runs(
    _client: Arc<Client>,
    _params: ValidatedGatherRuns,
) -> ToolResult<GatherRunsResult> {
    yield_now().await;
    Err(ToolError::message(
        "fabro_run_gather is not implemented yet",
    ))
}

pub(crate) async fn run_events(
    _client: Arc<Client>,
    _params: ValidatedRunEvents,
) -> ToolResult<RunEventsResult> {
    yield_now().await;
    Err(ToolError::message(
        "fabro_run_events is not implemented yet",
    ))
}

pub(crate) fn success_result<T: Serialize>(
    value: &T,
    text: impl Into<String>,
) -> Result<CallToolResult, rmcp::ErrorData> {
    let structured_content = serde_json::to_value(value).map_err(|err| {
        rmcp::ErrorData::internal_error(
            format!("failed to serialize Fabro MCP tool result: {err}"),
            None,
        )
    })?;
    let mut result = CallToolResult::structured(structured_content);
    result.content = vec![Content::text(text.into())];
    Ok(result)
}

pub(crate) fn error_result(err: ToolError) -> CallToolResult {
    CallToolResult::error(vec![Content::text(err.message)])
}

pub(crate) fn create_runs_text(result: &CreateRunsResult) -> String {
    let started = result.runs.iter().filter(|run| run.started).count();
    format!(
        "created {} Fabro run(s), started {started}",
        result.runs.len()
    )
}

pub(crate) fn search_runs_text(result: &SearchRunsResult) -> String {
    format!("found {} Fabro run(s)", result.runs.len())
}

pub(crate) fn interact_run_text(result: &InteractRunResult) -> String {
    format!(
        "completed {:?} for Fabro run {}",
        result.action, result.run_id
    )
}

pub(crate) fn gather_runs_text(result: &GatherRunsResult) -> String {
    format!(
        "gathered {} Fabro run(s), timed_out={}",
        result.runs.len(),
        result.timed_out
    )
}

pub(crate) fn run_events_text(result: &RunEventsResult) -> String {
    format!("returned {} Fabro event(s)", result.events.len())
}

fn validate_len(name: &str, len: usize, min: usize, max: usize) -> ToolResult<()> {
    if len < min {
        return Err(ToolError::message(format!(
            "{name} must contain at least {min} item(s)"
        )));
    }
    if len > max {
        return Err(ToolError::message(format!(
            "{name} must contain no more than {max} item(s)"
        )));
    }
    Ok(())
}

fn format_tool_error(err: &anyhow::Error) -> String {
    format!("{err:#}")
}
