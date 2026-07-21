//! Image model.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An image asset stored on disk and linked to songs.
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Image {
    pub id: Uuid,
    /// Publicly served URL for clients.
    pub public_url: String,
    /// Absolute filesystem path used for actual file.
    pub internal_path: Option<String>,
    pub credits: Option<String>,
}

/// Input for creating a new image record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewImage {
    pub public_url: String,
    pub internal_path: Option<String>,
    pub credits: Option<String>,
}

/// Input for replacing an image record's mutable fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateImage {
    pub public_url: String,
    pub internal_path: Option<String>,
    pub credits: Option<String>,
}
