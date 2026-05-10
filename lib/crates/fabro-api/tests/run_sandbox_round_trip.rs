use std::any::{TypeId, type_name};

use fabro_api::types::{
    RunSandbox as ApiRunSandbox, SandboxProvider as ApiSandboxProvider,
    SandboxResources as ApiSandboxResources,
};
use fabro_types::{RunSandbox, SandboxProvider, SandboxResources};
use serde_json::json;

#[test]
fn run_sandbox_reuses_domain_types() {
    assert_same_type::<ApiRunSandbox, RunSandbox>();
    assert_same_type::<ApiSandboxProvider, SandboxProvider>();
    assert_same_type::<ApiSandboxResources, SandboxResources>();
}

#[test]
fn run_sandbox_json_matches_openapi_shape() {
    let sandbox = RunSandbox {
        provider:          SandboxProvider::Docker,
        id:                "container-abc123".to_string(),
        working_directory: "/workspace".to_string(),
        repo_cloned:       Some(false),
        clone_origin_url:  Some("https://github.com/fabro-sh/fabro.git".to_string()),
        clone_branch:      Some("main".to_string()),
        resources:         Some(SandboxResources {
            cpu_cores:    Some(2.0),
            memory_bytes: Some(4 * 1024 * 1024 * 1024),
            disk_bytes:   None,
        }),
    };

    let value = serde_json::to_value(&sandbox).unwrap();

    assert_eq!(
        value,
        json!({
            "provider": "docker",
            "id": "container-abc123",
            "working_directory": "/workspace",
            "repo_cloned": false,
            "clone_origin_url": "https://github.com/fabro-sh/fabro.git",
            "clone_branch": "main",
            "resources": {
                "cpu_cores": 2.0,
                "memory_bytes": 4_294_967_296_u64,
            }
        })
    );
    assert!(value.get("identifier").is_none());
}

fn assert_same_type<T: 'static, U: 'static>() {
    assert_eq!(
        TypeId::of::<T>(),
        TypeId::of::<U>(),
        "{} should be the same type as {}",
        type_name::<T>(),
        type_name::<U>()
    );
}
