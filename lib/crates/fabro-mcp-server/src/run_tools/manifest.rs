use std::collections::HashMap;
use std::path::{Path, PathBuf};

use fabro_api::types;
use fabro_config::{CliLayer, RunLayer};
use fabro_manifest::{self, ManifestBuildInput, RunOverrideInput};
use fabro_server::manifest_validation;
use fabro_types::RunId;
use serde_json::Value;

use super::common::{ToolError, ToolResult};
use super::create::CreateRunSpec;

pub(super) fn build_mcp_run_manifest(
    spec: &CreateRunSpec,
    cwd: &Path,
    user_settings_path: &Path,
) -> ToolResult<types::RunManifest> {
    if let Some(run_id) = spec.run_id.as_deref() {
        run_id.parse::<RunId>().map_err(|err| {
            ToolError::message(format!("run_id must be a valid Fabro run id: {err}"))
        })?;
    }

    let built = fabro_manifest::build_run_manifest(ManifestBuildInput {
        workflow:           PathBuf::from(&spec.workflow),
        cwd:                cwd.to_path_buf(),
        run_overrides:      mcp_run_overrides(spec),
        cli_overrides:      Some(CliLayer::default()),
        input_overrides:    spec
            .inputs
            .iter()
            .map(|(key, value)| json_to_toml_value(key, value).map(|value| (key.clone(), value)))
            .collect::<ToolResult<HashMap<_, _>>>()?,
        args:               mcp_manifest_args(spec),
        run_id:             spec
            .run_id
            .as_deref()
            .map(str::parse::<RunId>)
            .transpose()
            .map_err(|err| {
                ToolError::message(format!("run_id must be a valid Fabro run id: {err}"))
            })?,
        user_settings_path: Some(user_settings_path.to_path_buf()),
    })
    .map_err(|err| ToolError::from_anyhow(&err))?;
    let validation = manifest_validation::validate_manifest(&RunLayer::default(), &built.manifest)
        .map_err(|err| ToolError::from_anyhow(&err))?;
    if !validation.ok {
        return Err(ToolError::message("workflow manifest validation failed"));
    }
    Ok(built.manifest)
}

pub(super) fn json_to_toml_value(key: &str, value: &Value) -> ToolResult<toml::Value> {
    match value {
        Value::Null => Err(ToolError::message(format!(
            "input `{key}` cannot be null; use a string, boolean, number, array, or object"
        ))),
        Value::Bool(value) => Ok(toml::Value::Boolean(*value)),
        Value::Number(value) => {
            if let Some(integer) = value.as_i64() {
                Ok(toml::Value::Integer(integer))
            } else if let Some(float) = value.as_f64() {
                Ok(toml::Value::Float(float))
            } else {
                Err(ToolError::message(format!(
                    "input `{key}` contains a number outside TOML's supported range"
                )))
            }
        }
        Value::String(value) => Ok(toml::Value::String(value.clone())),
        Value::Array(values) => values
            .iter()
            .map(|value| json_to_toml_value(key, value))
            .collect::<ToolResult<Vec<_>>>()
            .map(toml::Value::Array),
        Value::Object(values) => {
            let mut table = toml::Table::new();
            for (child_key, child_value) in values {
                table.insert(child_key.clone(), json_to_toml_value(key, child_value)?);
            }
            Ok(toml::Value::Table(table))
        }
    }
}

fn mcp_manifest_args(spec: &CreateRunSpec) -> Option<types::ManifestArgs> {
    let mut input = spec
        .inputs
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>();
    input.sort();
    let mut label = spec
        .labels
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>();
    label.sort();
    let payload = types::ManifestArgs {
        auto_approve: spec.auto_approve.filter(|value| *value),
        docker_image: None,
        dry_run: spec.dry_run.filter(|value| *value),
        input,
        label,
        model: spec.model.clone(),
        preserve_sandbox: spec.preserve_sandbox.filter(|value| *value),
        provider: spec.provider.clone(),
        sandbox: spec.sandbox.clone(),
        verbose: None,
    };
    (!fabro_manifest::manifest_args_is_empty(&payload)).then_some(payload)
}

fn mcp_run_overrides(spec: &CreateRunSpec) -> Option<RunLayer> {
    fabro_manifest::build_sparse_run_overrides(RunOverrideInput {
        goal:             spec.goal.as_deref(),
        model:            spec.model.as_deref(),
        provider:         spec.provider.as_deref(),
        sandbox:          spec.sandbox.as_deref(),
        preserve_sandbox: spec.preserve_sandbox,
        dry_run:          spec.dry_run,
        auto_approve:     spec.auto_approve,
        labels:           spec.labels.clone(),
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::{Value, json};

    use super::super::create::CreateRunSpec;
    use super::*;

    #[test]
    fn json_inputs_convert_to_toml_values() {
        let cases = [
            (json!("hello"), toml::Value::String("hello".to_string())),
            (json!(true), toml::Value::Boolean(true)),
            (json!(42), toml::Value::Integer(42)),
            (json!(0.5), toml::Value::Float(0.5)),
            (
                json!(["a", 1]),
                toml::Value::Array(vec![
                    toml::Value::String("a".to_string()),
                    toml::Value::Integer(1),
                ]),
            ),
            (
                json!({ "enabled": true, "count": 2 }),
                toml::Value::Table(toml::Table::from_iter([
                    ("enabled".to_string(), toml::Value::Boolean(true)),
                    ("count".to_string(), toml::Value::Integer(2)),
                ])),
            ),
        ];

        for (json, expected) in cases {
            assert_eq!(json_to_toml_value("input", &json).unwrap(), expected);
        }
    }

    #[test]
    fn json_input_null_is_rejected_with_key_name() {
        let err = json_to_toml_value("goal", &Value::Null).unwrap_err();

        assert!(err.as_str().contains("goal"));
        assert!(err.as_str().contains("null"));
    }

    #[test]
    fn mcp_manifest_args_preserve_input_provenance() {
        let args = mcp_manifest_args(&CreateRunSpec {
            workflow:         "simple".to_string(),
            run_id:           None,
            cwd:              None,
            goal:             None,
            inputs:           HashMap::from([
                ("count".to_string(), json!(3)),
                ("decision".to_string(), json!("approve")),
            ]),
            labels:           HashMap::new(),
            model:            None,
            provider:         None,
            sandbox:          None,
            dry_run:          None,
            auto_approve:     None,
            preserve_sandbox: None,
            start:            None,
        })
        .expect("input args should be present");

        assert_eq!(args.input, vec![r"count=3", r#"decision="approve""#]);
    }
}
