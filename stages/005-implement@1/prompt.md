Goal: # Define automation API contract and clients

- Number: #396
- State: OPEN
- Author: brynary (Bryan Helmkamp)
- Created: 2026-05-25T15:06:24Z
- Updated: 2026-05-25T15:06:24Z
- URL: https://github.com/fabro-sh/fabro/issues/396

## Body

## Goal

Define the public Automations API contract, reuse compatible Rust domain types, and regenerate generated API clients.

## Scope

Update the OpenAPI spec under `/api/v1` with these paths:

```http
GET    /automations
POST   /automations
GET    /automations/{id}
PUT    /automations/{id}
DELETE /automations/{id}
GET    /automations/{id}/runs
POST   /automations/{id}/runs
```

Add schemas for:

- `Automation`
- `AutomationTarget`
- `AutomationTrigger`
- `AutomationApiTrigger`
- `AutomationScheduleTrigger`
- `CreateAutomationRequest`
- `ReplaceAutomationRequest`
- `AutomationListResponse`

Use this response shape for automations:

```ts
type Automation = {
  id: string;
  revision: string;
  name: string;
  description: string | null;
  enabled: boolean;
  target: AutomationTarget;
  triggers: AutomationTrigger[];
};

type AutomationTarget = {
  repository: string;
  ref: string;
  workflow: string;
};

type AutomationTrigger =
  | { id: string; type: "api"; enabled: boolean }
  | { id: string; type: "schedule"; enabled: boolean; expression: string };
```

Request shapes:

```ts
type CreateAutomationRequest = {
  id: string;
  name: string;
  description?: string | null;
  enabled?: boolean;
  target: AutomationTarget;
  triggers: AutomationTrigger[];
};

type ReplaceAutomationRequest = {
  name: string;
  description?: string | null;
  enabled: boolean;
  target: AutomationTarget;
  triggers: AutomationTrigger[];
};
```

Contract details:

- Use an OpenAPI discriminator with `propertyName: type` for trigger variants.
- Unknown trigger discriminator values should be reported by handlers as domain validation errors with HTTP 422, not malformed JSON errors with HTTP 400.
- Reuse existing `Run` and paginated run-list envelope schemas for automation run endpoints.
- Add `If-Match` header parameters for replace and delete operations.
- Add `ETag` response headers on automation read and replace responses.
- Response codes should cover 200, 201, 204, 400, 404, 409, 422, and 428 where applicable.
- Before generating duplicate Rust types, search for matching `fabro-automation` domain types and add `with_replacement(...)` mappings where the serde wire shape is identical.
- Add JSON parity tests for every reused automation type.

## Files

Modify:

- `docs/public/api-reference/fabro-api.yaml`
- `lib/crates/fabro-api/Cargo.toml`
- `lib/crates/fabro-api/build.rs`
- Generated Rust API files under `lib/crates/fabro-api/src`, as produced by build/codegen
- Generated TypeScript client files under `lib/packages/fabro-api-client`

Create:

- `lib/crates/fabro-api/tests/automation_round_trip.rs`

## Acceptance Criteria

- The OpenAPI document exposes the full backend Automations API contract.
- Rust generated API types compile.
- Compatible automation domain types are reused instead of duplicated.
- Reused type JSON parity is covered by tests.
- The TypeScript API client contains generated automation operations and types.
- No web UI consumer imports are added.

## Verification

Run:

```bash
cargo build -p fabro-api
cargo nextest run -p fabro-api
cd lib/packages/fabro-api-client && bun run generate
git diff -- apps/fabro-web lib/crates/fabro-cli
```

Expected: generated API files may change; application UI and CLI command modules should not change.


## Comments

No comments.


## Completed stages
- **toolchain**: succeeded
  - Script: `command -v cargo >/dev/null || { curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && sudo ln -sf $HOME/.cargo/bin/* /usr/local/bin/; }; cargo --version 2>&1`
  - Output:
    ```
    cargo 1.95.0 (f2d3ce0bd 2026-03-21)
    ```
- **preflight_compile**: succeeded
  - Script: `cargo check -q --workspace 2>&1`
  - Output: (empty)
- **preflight_lint**: succeeded
  - Script: `cargo +nightly-2026-04-14 clippy -q --workspace --all-targets -- -D warnings 2>&1`
  - Output: (empty)


Read the plan file referenced in the goal and implement every step. Make all the code changes described in the plan. Use red/green TDD.