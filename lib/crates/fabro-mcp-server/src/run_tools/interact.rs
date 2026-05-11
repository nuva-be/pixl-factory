use std::sync::Arc;

use fabro_api::types;
use fabro_client::Client;
use fabro_types::RunId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::common;
use super::common::{ToolError, ToolResult};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, JsonSchema)]
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
            let Some(answer) = params.answer.as_ref() else {
                return Err(ToolError::message("answer is required for action answer"));
            };
            answer_to_submit_request(answer.clone())?;
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

pub(crate) async fn interact_run(
    client: Arc<Client>,
    params: ValidatedInteractRun,
) -> ToolResult<InteractRunResult> {
    let raw = params.raw;
    let run_id = client
        .resolve_run(&raw.run_id)
        .await
        .map_err(|err| ToolError::from_anyhow(&err))?
        .id;
    let result = match raw.action {
        RunInteractAction::Get => interact_get(&client, &run_id).await?,
        RunInteractAction::Start => {
            client
                .start_run(&run_id, false)
                .await
                .map_err(|err| ToolError::from_anyhow(&err))?;
            json!({ "summary": common::run_summary_result(&common::retrieve_run(&client, &run_id).await?) })
        }
        RunInteractAction::Message => {
            let message = raw
                .message
                .expect("validated message action has a message")
                .trim()
                .to_string();
            client
                .steer_run(&run_id, message.clone(), raw.interrupt.unwrap_or(false))
                .await
                .map_err(|err| ToolError::from_anyhow(&err))?;
            json!({ "message": message, "interrupt": raw.interrupt.unwrap_or(false) })
        }
        RunInteractAction::Cancel => {
            client
                .cancel_run(&run_id)
                .await
                .map_err(|err| ToolError::from_anyhow(&err))?;
            json!({ "summary": common::run_summary_result(&common::retrieve_run(&client, &run_id).await?) })
        }
        RunInteractAction::Archive => {
            client
                .archive_run(&run_id)
                .await
                .map_err(|err| ToolError::from_anyhow(&err))?;
            json!({ "summary": common::run_summary_result(&common::retrieve_run(&client, &run_id).await?) })
        }
        RunInteractAction::Unarchive => {
            client
                .unarchive_run(&run_id)
                .await
                .map_err(|err| ToolError::from_anyhow(&err))?;
            json!({ "summary": common::run_summary_result(&common::retrieve_run(&client, &run_id).await?) })
        }
        RunInteractAction::GetQuestions => {
            let questions = client
                .list_run_questions(&run_id)
                .await
                .map_err(|err| ToolError::from_anyhow(&err))?;
            json!({ "questions": questions })
        }
        RunInteractAction::Answer => {
            let question_id = raw
                .question_id
                .expect("validated answer action has a question_id");
            let body = answer_to_submit_request(
                raw.answer.expect("validated answer action has an answer"),
            )?;
            client
                .submit_run_answer(&run_id, &question_id, body)
                .await
                .map_err(|err| ToolError::from_anyhow(&err))?;
            json!({ "question_id": question_id, "submitted": true })
        }
    };

    Ok(InteractRunResult {
        run_id: run_id.to_string(),
        action: raw.action,
        result,
    })
}

pub(crate) fn interact_run_text(result: &InteractRunResult) -> String {
    format!(
        "completed {:?} for Fabro run {}",
        result.action, result.run_id
    )
}

async fn interact_get(client: &Client, run_id: &RunId) -> ToolResult<Value> {
    let summary = common::retrieve_run(client, run_id).await?;
    let projection = client
        .get_run_state(run_id)
        .await
        .map_err(|err| ToolError::from_anyhow(&err))?;
    Ok(json!({
        "summary": common::run_summary_result(&summary),
        "projection": projection,
    }))
}

fn answer_to_submit_request(answer: Value) -> ToolResult<types::SubmitAnswerRequest> {
    let payload = match answer {
        Value::Bool(true) => json!({ "kind": "yes" }),
        Value::Bool(false) => json!({ "kind": "no" }),
        Value::String(text) => json!({ "kind": "text", "text": text }),
        Value::Object(mut object) => {
            if let Some(option) = object.remove("option") {
                json!({ "kind": "selected", "option_key": option })
            } else if let Some(options) = object.remove("options") {
                json!({ "kind": "multi_selected", "option_keys": options })
            } else if let Some(text) = object.remove("text") {
                json!({ "kind": "text", "text": text })
            } else {
                return Err(ToolError::message(
                    "answer object must contain one of: option, options, text",
                ));
            }
        }
        other => {
            return Err(ToolError::message(format!(
                "unsupported answer value: {other}; expected boolean, string, or object",
            )));
        }
    };
    serde_json::from_value(payload)
        .map_err(|err| ToolError::message(format!("failed to build submit-answer request: {err}")))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn answer_payloads_map_to_submit_answer_wire_json() {
        let cases = [
            (json!(true), json!({ "kind": "yes" })),
            (json!(false), json!({ "kind": "no" })),
            (json!("hello"), json!({ "kind": "text", "text": "hello" })),
            (
                json!({ "option": "a" }),
                json!({ "kind": "selected", "option_key": "a" }),
            ),
            (
                json!({ "options": ["a", "b"] }),
                json!({ "kind": "multi_selected", "option_keys": ["a", "b"] }),
            ),
            (
                json!({ "text": "hello" }),
                json!({ "kind": "text", "text": "hello" }),
            ),
        ];

        for (answer, expected) in cases {
            let request = answer_to_submit_request(answer).unwrap();
            assert_eq!(serde_json::to_value(request).unwrap(), expected);
        }
    }

    #[test]
    fn unsupported_answer_object_is_rejected() {
        let err = answer_to_submit_request(json!({ "value": "yes" })).unwrap_err();

        assert!(err.as_str().contains("option, options, text"));
    }

    #[test]
    fn interact_answer_validation_rejects_unsupported_json_before_api_calls() {
        let err = ValidatedInteractRun::try_from(FabroRunInteractParams {
            action:      RunInteractAction::Answer,
            run_id:      "run_123".to_string(),
            message:     None,
            interrupt:   None,
            question_id: Some("question-1".to_string()),
            answer:      Some(json!({ "value": "yes" })),
        })
        .unwrap_err();

        assert!(err.as_str().contains("option, options, text"));
    }
}
