use serde::{Deserialize, Serialize};

use crate::{SandboxProvider, SandboxResources};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunSandbox {
    pub provider:          SandboxProvider,
    pub id:                String,
    pub working_directory: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repo_cloned:       Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clone_origin_url:  Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clone_branch:      Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources:         Option<SandboxResources>,
}
