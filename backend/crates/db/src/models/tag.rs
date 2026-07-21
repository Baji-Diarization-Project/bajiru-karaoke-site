//! Tag model.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A tag record from the `tags` table.
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
}

/// A tag joined with its kind from an entity-specific join table.
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct TagWithKind {
    pub id: Uuid,
    pub name: String,
    pub kind: String,
}

/// Input for creating or looking up a tag by name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTag {
    pub name: String,
}
