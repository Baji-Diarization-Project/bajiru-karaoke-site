//! Lyrics model.
//!
//! Lyrics rows are shared: a single row can be referenced by multiple songs
//! and performances. See `queries::lyrics::reference_count` for safe deletion.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A lyrics row fetched from the database.
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Lyrics {
    pub id: Uuid,
    pub content: String,
}

/// Input for creating a new lyrics row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewLyrics {
    pub content: String,
}
