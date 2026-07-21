//! Authentication request and response types.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Request body for `POST /auth/register`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    /// Alphanumeric with `_` and `.` allowed, max 64 chars, case insensitive unique.
    pub username: String,
    /// Minimum 8 characters.
    pub password: String,
}

/// Request body for `POST /auth/login`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Response body for `GET /auth/me`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MeResponse {
    pub id: Uuid,
    pub username: String,
    /// Capability titles embedded in the session JWT.
    pub capabilities: Vec<String>,
}
