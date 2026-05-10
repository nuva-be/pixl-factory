use std::any::{TypeId, type_name};
use std::collections::BTreeMap;

use chrono::{TimeZone, Utc};
use fabro_api::types::{
    SandboxDetails as ApiSandboxDetails, SandboxProvider as ApiSandboxProvider,
    SandboxResources as ApiSandboxResources, SandboxState as ApiSandboxState,
    SandboxTimestamps as ApiSandboxTimestamps,
};
use fabro_types::{
    SandboxDetails, SandboxProvider, SandboxResources, SandboxState, SandboxTimestamps,
};
use serde_json::json;

#[test]
fn sandbox_details_reuses_domain_types() {
    assert_same_type::<ApiSandboxDetails, SandboxDetails>();
    assert_same_type::<ApiSandboxProvider, SandboxProvider>();
    assert_same_type::<ApiSandboxState, SandboxState>();
    assert_same_type::<ApiSandboxResources, SandboxResources>();
    assert_same_type::<ApiSandboxTimestamps, SandboxTimestamps>();
}

#[test]
fn sandbox_details_json_matches_openapi_shape() {
    let created_at = Utc.with_ymd_and_hms(2026, 5, 9, 12, 0, 0).unwrap();
    let details = SandboxDetails {
        provider:          SandboxProvider::Docker,
        id:                "container-abc123".to_string(),
        working_directory: "/workspace".to_string(),
        state:             SandboxState::Running,
        native_state:      Some("running".to_string()),
        region:            None,
        image:             Some("ghcr.io/fabro/sandbox:latest".to_string()),
        resources:         SandboxResources {
            cpu_cores:    Some(2.0),
            memory_bytes: Some(4 * 1024 * 1024 * 1024),
            disk_bytes:   None,
        },
        labels:            BTreeMap::from([("run".to_string(), "abc".to_string())]),
        timestamps:        SandboxTimestamps {
            created_at:       Some(created_at),
            last_activity_at: None,
        },
    };

    assert_eq!(
        serde_json::to_value(&details).unwrap(),
        json!({
            "provider": "docker",
            "id": "container-abc123",
            "working_directory": "/workspace",
            "state": "running",
            "native_state": "running",
            "image": "ghcr.io/fabro/sandbox:latest",
            "resources": {
                "cpu_cores": 2.0,
                "memory_bytes": 4_294_967_296_u64,
            },
            "labels": {
                "run": "abc"
            },
            "timestamps": {
                "created_at": "2026-05-09T12:00:00Z"
            }
        })
    );
}

#[test]
fn sandbox_details_deserializes_when_optional_fields_are_absent() {
    let details: SandboxDetails = serde_json::from_value(json!({
        "provider": "local",
        "id": "local:01JNQVR7M0EJ5GKAT2SC4ERS1Z",
        "working_directory": "/Users/client/project",
        "state": "unknown",
        "resources": {},
        "labels": {},
        "timestamps": {}
    }))
    .unwrap();

    assert_eq!(details.provider, SandboxProvider::Local);
    assert_eq!(details.id, "local:01JNQVR7M0EJ5GKAT2SC4ERS1Z");
    assert_eq!(details.working_directory, "/Users/client/project");
    assert_eq!(details.state, SandboxState::Unknown);
    assert!(details.image.is_none());
    assert!(details.region.is_none());
    assert!(details.native_state.is_none());
    assert!(details.labels.is_empty());
    assert_eq!(details.resources, SandboxResources::default());
    assert_eq!(details.timestamps, SandboxTimestamps::default());
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
