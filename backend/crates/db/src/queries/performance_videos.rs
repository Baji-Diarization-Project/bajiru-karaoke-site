//! Query functions for the `performance_videos` table.

use sqlx::{Executor, MySql, MySqlConnection};
use uuid::Uuid;

use crate::error::DbError;
use crate::models::performance_video::{NewPerformanceVideo, PerformanceVideo};

type Result<T> = std::result::Result<T, DbError>;

/// Fetches a performance video record by ID.
pub async fn get_by_id(
    executor: impl Executor<'_, Database = MySql>,
    id: Uuid,
) -> Result<Option<PerformanceVideo>> {
    sqlx::query_as::<_, PerformanceVideo>(
        "SELECT id, performance_id, public_url, internal_path \
         FROM performance_videos WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(executor)
    .await
    .map_err(DbError::from)
}

/// Returns all video records for a given performance.
pub async fn list_for_performance(
    executor: impl Executor<'_, Database = MySql>,
    performance_id: Uuid,
) -> Result<Vec<PerformanceVideo>> {
    sqlx::query_as::<_, PerformanceVideo>(
        "SELECT id, performance_id, public_url, internal_path \
         FROM performance_videos WHERE performance_id = ?",
    )
    .bind(performance_id)
    .fetch_all(executor)
    .await
    .map_err(DbError::from)
}

/// Inserts a new performance video record and returns it.
pub async fn create(
    conn: &mut MySqlConnection,
    new: &NewPerformanceVideo,
) -> Result<PerformanceVideo> {
    sqlx::query_as::<_, PerformanceVideo>(
        "INSERT INTO performance_videos (performance_id, public_url, internal_path) \
         VALUES (?, ?, ?) \
         RETURNING id, performance_id, public_url, internal_path",
    )
    .bind(new.performance_id)
    .bind(&new.public_url)
    .bind(&new.internal_path)
    .fetch_one(conn)
    .await
    .map_err(DbError::from)
}

/// Deletes a performance video record by ID. Returns `true` if a row was deleted.
pub async fn delete(executor: impl Executor<'_, Database = MySql>, id: Uuid) -> Result<bool> {
    sqlx::query("DELETE FROM performance_videos WHERE id = ?")
        .bind(id)
        .execute(executor)
        .await
        .map(|r| r.rows_affected() > 0)
        .map_err(DbError::from)
}
