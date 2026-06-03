//! Test-only helpers for constructing total `Principal` and `RunProvenance`
//! values without requiring callers to assemble fake identity bits inline.
//!
//! Only available behind `#[cfg(test)]` or the `test-support` feature. Do not
//! use these in production code or release builds.

use crate::{AuthMethod, IdpIdentity, Principal, RunProvenance};

/// A clearly synthetic dev-token principal for tests.
#[must_use]
pub fn test_principal() -> Principal {
    Principal::user(
        IdpIdentity::new("fabro:test", "test-user").expect("test identity should parse"),
        "test".to_string(),
        AuthMethod::DevToken,
    )
}

/// A `RunProvenance` with a synthetic test subject and no server/client
/// provenance fields.
#[must_use]
pub fn test_run_provenance() -> RunProvenance {
    RunProvenance {
        server:  None,
        client:  None,
        subject: test_principal(),
    }
}
