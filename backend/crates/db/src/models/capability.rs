//! Capability model.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A named permission label that can be granted to users.
///
/// Capability titles (e.g. `"admin"`) are embedded in the session JWT and
/// checked by route handlers to gate privileged operations.
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Capability {
    pub id: Uuid,
    pub title: String,
}

/// Input for creating a new capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCapability {
    pub title: String,
}
