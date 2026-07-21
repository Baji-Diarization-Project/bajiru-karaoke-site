//! Performance audio model.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An audio file attached to a performance.
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct PerformanceAudio {
    pub id: Uuid,
    pub performance_id: Uuid,
    /// Publicly served URL for clients.
    pub public_url: String,
    /// Absolute filesystem path used for actual file.
    pub internal_path: Option<String>,
}

/// Input for creating a new performance audio record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPerformanceAudio {
    pub performance_id: Uuid,
    pub public_url: String,
    pub internal_path: Option<String>,
}
