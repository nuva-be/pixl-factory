use std::collections::HashMap;
use std::sync::Arc;

use fabro_client::Client;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::common;
use super::common::{RunSummaryResult, ToolError, ToolResult};

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
        if let Some(created_after) = params.created_after.as_deref() {
            common::parse_datetime_filter("created_after", created_after)?;
        }
        if let Some(created_before) = params.created_before.as_deref() {
            common::parse_datetime_filter("created_before", created_before)?;
        }
        Ok(Self { raw: params })
    }
}

#[derive(Debug, Serialize, JsonSchema)]
pub(crate) struct SearchRunsResult {
    pub(crate) runs:        Vec<RunSummaryResult>,
    pub(crate) next_cursor: Option<String>,
}

pub(crate) async fn search_runs(
    client: Arc<Client>,
    params: ValidatedSearchRuns,
) -> ToolResult<SearchRunsResult> {
    let raw = params.raw;
    let mut runs = client
        .list_store_runs()
        .await
        .map_err(|err| ToolError::from_anyhow(&err))?;
    runs.sort_by(|a, b| {
        let a_sort_time = a.timestamps.started_at.unwrap_or(a.timestamps.created_at);
        let b_sort_time = b.timestamps.started_at.unwrap_or(b.timestamps.created_at);
        b_sort_time.cmp(&a_sort_time).then_with(|| b.id.cmp(&a.id))
    });

    if let Some(after) = raw.after.as_deref() {
        if let Some(position) = runs.iter().position(|run| run.id.to_string() == after) {
            runs = runs.into_iter().skip(position + 1).collect();
        }
    }

    if let Some(run_ids) = raw.run_ids.as_ref() {
        runs.retain(|run| run_ids.iter().any(|id| id == &run.id.to_string()));
    }
    if let Some(workflow) = raw.workflow.as_deref() {
        runs.retain(|run| {
            run.workflow.name == workflow || run.workflow.slug.as_deref() == Some(workflow)
        });
    }
    if let Some(labels) = raw.labels.as_ref() {
        runs.retain(|run| {
            labels
                .iter()
                .all(|(key, value)| run.labels.get(key) == Some(value))
        });
    }
    if let Some(status) = raw.status.as_ref() {
        runs.retain(|run| {
            status
                .iter()
                .any(|status| status == common::run_status_kind(run.lifecycle.status))
        });
    }
    if let Some(archived) = raw.archived {
        runs.retain(|run| run.lifecycle.archived == archived);
    }
    if let Some(created_after) = raw.created_after.as_deref() {
        let cutoff = common::parse_datetime_filter("created_after", created_after)?;
        runs.retain(|run| run.timestamps.created_at >= cutoff);
    }
    if let Some(created_before) = raw.created_before.as_deref() {
        let cutoff = common::parse_datetime_filter("created_before", created_before)?;
        runs.retain(|run| run.timestamps.created_at <= cutoff);
    }

    let first = raw.first.unwrap_or(20).min(100);
    let has_more = runs.len() > first;
    let page = runs.into_iter().take(first).collect::<Vec<_>>();
    let next_cursor = has_more
        .then(|| page.last().map(|run| run.id.to_string()))
        .flatten();
    Ok(SearchRunsResult {
        runs: page.iter().map(common::run_summary_result).collect(),
        next_cursor,
    })
}

pub(crate) fn search_runs_text(result: &SearchRunsResult) -> String {
    format!("found {} Fabro run(s)", result.runs.len())
}
