//! Artist request and response types.
//!
//! Updates use PUT semantics: all fields are required and missing optionals mean null.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// An artist record returned by artist endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ArtistResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

/// Request body for `POST /api/artists`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateArtistRequest {
    pub name: String,
    pub description: Option<String>,
}

/// Request body for `PUT /api/artists/{id}`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateArtistRequest {
    pub name: String,
    pub description: Option<String>,
}
