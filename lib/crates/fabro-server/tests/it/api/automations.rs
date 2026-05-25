use axum::body::Body;
use axum::http::{Method, Request, StatusCode, header};
use fabro_server::test_support::{TestAppStateBuilder, build_test_router};
use serde_json::{Value, json};
use tower::ServiceExt;

use crate::helpers::{MINIMAL_DOT, api, minimal_manifest_json, response_json, response_status};

fn automation_body(id: &str) -> Value {
    json!({
        "id": id,
        "name": "Nightly dependency update",
        "description": "Open a PR for dependency updates.",
        "target": {
            "repository": "fabro-sh/fabro",
            "ref": "main",
            "workflow": "dependency-update"
        },
        "triggers": [
            { "id": "api", "type": "api", "enabled": true },
            { "id": "nightly", "type": "schedule", "enabled": true, "expression": "0 3 * * *" }
        ]
    })
}

fn request_json(method: Method, path: &str, body: &Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(api(path))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .expect("request should build")
}

async fn create_automation(app: &axum::Router, id: &str) -> Value {
    let response = app
        .clone()
        .oneshot(request_json(
            Method::POST,
            "/automations",
            &automation_body(id),
        ))
        .await
        .unwrap();
    response_json(response, StatusCode::CREATED, "POST /automations").await
}

#[tokio::test]
async fn empty_list_returns_total_zero() {
    let app = build_test_router(TestAppStateBuilder::new().build());

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(api("/automations"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response_json(response, StatusCode::OK, "GET /automations").await;
    assert_eq!(body, json!({ "data": [], "meta": { "total": 0 } }));
}

#[tokio::test]
async fn create_writes_toml_and_duplicate_conflicts() {
    let dir = tempfile::tempdir().unwrap();
    let state = TestAppStateBuilder::new()
        .active_config_path(dir.path().join("settings.toml"))
        .build();
    let app = build_test_router(state);

    let body = create_automation(&app, "nightly-deps").await;

    assert_eq!(body["id"], "nightly-deps");
    assert_eq!(body["enabled"], true);
    assert!(dir.path().join("automations/nightly-deps.toml").exists());

    let response = app
        .clone()
        .oneshot(request_json(
            Method::POST,
            "/automations",
            &automation_body("nightly-deps"),
        ))
        .await
        .unwrap();
    response_status(response, StatusCode::CONFLICT, "duplicate automation").await;
}

#[tokio::test]
async fn get_replace_patch_and_delete_use_etags() {
    let app = build_test_router(TestAppStateBuilder::new().build());
    let created = create_automation(&app, "nightly-deps").await;
    let revision = created["revision"].as_str().unwrap().to_string();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(api("/automations/nightly-deps"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.headers()[header::ETAG], format!("\"{revision}\""));
    let got = response_json(response, StatusCode::OK, "GET automation").await;
    assert_eq!(got["id"], "nightly-deps");

    let replace = json!({
        "name": "Updated automation",
        "description": "updated",
        "enabled": true,
        "target": automation_body("ignored")["target"].clone(),
        "triggers": [{ "id": "api", "type": "api", "enabled": true }]
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri(api("/automations/nightly-deps"))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::IF_MATCH, revision.clone())
                .body(Body::from(replace.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let replaced = response_json(response, StatusCode::OK, "PUT automation").await;
    assert_eq!(replaced["name"], "Updated automation");
    let new_revision = replaced["revision"].as_str().unwrap().to_string();

    let stale_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri(api("/automations/nightly-deps"))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::IF_MATCH, revision)
                .body(Body::from(replace.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    response_status(stale_response, StatusCode::CONFLICT, "stale replace").await;

    let missing_if_match = app
        .clone()
        .oneshot(request_json(
            Method::PATCH,
            "/automations/nightly-deps",
            &json!({ "description": null }),
        ))
        .await
        .unwrap();
    response_status(
        missing_if_match,
        StatusCode::PRECONDITION_REQUIRED,
        "missing if-match",
    )
    .await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::PATCH)
                .uri(api("/automations/nightly-deps"))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::IF_MATCH, format!("\"{new_revision}\""))
                .body(Body::from(json!({ "description": null }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let patched = response_json(response, StatusCode::OK, "PATCH automation").await;
    assert_eq!(patched["description"], Value::Null);
    let patched_revision = patched["revision"].as_str().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri(api("/automations/nightly-deps"))
                .header(header::IF_MATCH, patched_revision)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    response_status(response, StatusCode::NO_CONTENT, "DELETE automation").await;
}

#[tokio::test]
async fn validation_errors_return_422() {
    let app = build_test_router(TestAppStateBuilder::new().build());
    for (label, triggers) in [
        (
            "invalid trigger id",
            json!([{ "id": "_api", "type": "api", "enabled": true }]),
        ),
        (
            "duplicate trigger id",
            json!([
                { "id": "api", "type": "api", "enabled": true },
                { "id": "api", "type": "schedule", "enabled": true, "expression": "0 3 * * *" }
            ]),
        ),
        (
            "second api trigger",
            json!([
                { "id": "api", "type": "api", "enabled": true },
                { "id": "api2", "type": "api", "enabled": true }
            ]),
        ),
        (
            "invalid schedule",
            json!([{ "id": "nightly", "type": "schedule", "enabled": true, "expression": "* * * * * *" }]),
        ),
        (
            "unknown trigger",
            json!([{ "id": "event", "type": "event", "enabled": true }]),
        ),
        (
            "unknown trigger future shape",
            json!([{ "id": "event", "type": "event", "enabled": true, "pattern": "push" }]),
        ),
    ] {
        let mut body = automation_body(label.replace(' ', "-").as_str());
        body["triggers"] = triggers;
        let response = app
            .clone()
            .oneshot(request_json(Method::POST, "/automations", &body))
            .await
            .unwrap();
        response_status(response, StatusCode::UNPROCESSABLE_ENTITY, label).await;
    }
}

#[tokio::test]
async fn disabled_or_missing_enabled_api_trigger_cannot_start() {
    let app = build_test_router(TestAppStateBuilder::new().build());
    let mut disabled_automation = automation_body("disabled");
    disabled_automation["enabled"] = json!(false);
    response_json(
        app.clone()
            .oneshot(request_json(
                Method::POST,
                "/automations",
                &disabled_automation,
            ))
            .await
            .unwrap(),
        StatusCode::CREATED,
        "create disabled automation",
    )
    .await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(api("/automations/disabled/runs"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = response_json(response, StatusCode::CONFLICT, "start disabled automation").await;
    assert_eq!(body["errors"][0]["code"], "automation_api_trigger_disabled");

    let mut disabled_trigger = automation_body("disabled-trigger");
    disabled_trigger["triggers"] = json!([{ "id": "api", "type": "api", "enabled": false }]);
    response_json(
        app.clone()
            .oneshot(request_json(
                Method::POST,
                "/automations",
                &disabled_trigger,
            ))
            .await
            .unwrap(),
        StatusCode::CREATED,
        "create disabled trigger automation",
    )
    .await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(api("/automations/disabled-trigger/runs"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    response_status(response, StatusCode::CONFLICT, "start disabled trigger").await;
}

#[tokio::test]
async fn api_triggered_run_persists_automation_and_lists_runs() {
    let manifest: fabro_api::types::RunManifest =
        serde_json::from_value(minimal_manifest_json(MINIMAL_DOT))
            .expect("minimal manifest should deserialize");
    let state = TestAppStateBuilder::new()
        .automation_materializer_manifest(manifest)
        .build();
    let app = build_test_router(state);
    create_automation(&app, "nightly-deps").await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(api("/automations/nightly-deps/runs"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let run = response_json(response, StatusCode::CREATED, "POST automation run").await;
    assert_eq!(run["automation"]["id"], "nightly-deps");
    assert_eq!(run["automation"]["name"], "Nightly dependency update");
    assert_eq!(run["automation"]["trigger_id"], "api");

    let run_id = run["id"].as_str().unwrap();
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(api(&format!("/runs/{run_id}")))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let persisted = response_json(response, StatusCode::OK, "GET run").await;
    assert_eq!(persisted["automation"], run["automation"]);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(api(
                    "/automations/nightly-deps/runs?page[limit]=10&page[offset]=0",
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let runs = response_json(response, StatusCode::OK, "GET automation runs").await;
    assert_eq!(runs["meta"], json!({ "has_more": false, "total": 1 }));
    assert_eq!(runs["data"][0]["id"], run_id);
}
