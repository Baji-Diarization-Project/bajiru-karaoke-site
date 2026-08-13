//! Playlist model.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A playlist of performances.
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Playlist {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub kind: String,
    pub is_public: bool,
    /// User who created this playlist. `None` for system generated playlists.
    pub created_by: Option<Uuid>,
}

/// Input for creating a new playlist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPlaylist {
    pub title: String,
    pub description: Option<String>,
    pub kind: String,
    pub is_public: bool,
    pub created_by: Option<Uuid>,
}

/// Input for replacing a playlist's mutable fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePlaylist {
    pub title: String,
    pub description: Option<String>,
    pub kind: String,
    pub is_public: bool,
}
