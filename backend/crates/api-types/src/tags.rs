//! Tag resource types and per-entity kind enums.
//!
//! Tags are a shared name pool. Kind is absent from the tag itself and lives on
//! each entity's join table, enforced at deserialization.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// A tag record returned by tag endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TagResponse {
    pub id: Uuid,
    pub name: String,
}

/// Request body for `POST /api/tags`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateTagRequest {
    pub name: String,
}

/// Valid kind values when applying a tag to a song.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SongTagKind {
    Genre,
    /// Where the song originates from (anime, game, original, vocaloid, etc.).
    Source,
    Language,
    Misc,
}

impl SongTagKind {
    /// Returns the string stored in the database for this kind.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Genre => "genre",
            Self::Source => "source",
            Self::Language => "language",
            Self::Misc => "misc",
        }
    }
}

/// Valid kind values when applying a tag to a performance.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PerformanceTagKind {
    /// Instrument used in the performance (e.g. ukulele, piano).
    Instrument,
    /// A stylistic or arrangement variation of the song (e.g. uke, slow version).
    Modifier,
    Misc,
}

impl PerformanceTagKind {
    /// Returns the string stored in the database for this kind.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Instrument => "instrument",
            Self::Modifier => "modifier",
            Self::Misc => "misc",
        }
    }
}
