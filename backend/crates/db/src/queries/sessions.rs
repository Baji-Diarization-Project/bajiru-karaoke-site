//! Query functions for the `sessions` table.

use chrono::{DateTime, Utc};
use sqlx::{Executor, MySql};
use uuid::Uuid;

use crate::error::DbError;
use crate::models::session::Session;

type Result<T> = std::result::Result<T, DbError>;

/// Inserts a new session row. `id` is the SHA256 hash of the session token.
pub async fn create(
    executor: impl Executor<'_, Database = MySql>,
    id: &str,
    user_id: Uuid,
    expires_at: DateTime<Utc>,
) -> Result<()> {
    sqlx::query("INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, ?)")
        .bind(id)
        .bind(user_id)
        .bind(expires_at)
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(DbError::from)
}

/// Fetches a session by ID, returning `None` if it is missing or expired.
pub async fn get_valid(
    executor: impl Executor<'_, Database = MySql>,
    id: &str,
) -> Result<Option<Session>> {
    sqlx::query_as::<_, Session>(
        "SELECT id, user_id, created_at, expires_at FROM sessions \
         WHERE id = ? AND expires_at > UTC_TIMESTAMP()",
    )
    .bind(id)
    .fetch_optional(executor)
    .await
    .map_err(DbError::from)
}

/// Deletes a session by ID. Missing IDs are a no-op.
pub async fn delete(executor: impl Executor<'_, Database = MySql>, id: &str) -> Result<()> {
    sqlx::query("DELETE FROM sessions WHERE id = ?")
        .bind(id)
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(DbError::from)
}
