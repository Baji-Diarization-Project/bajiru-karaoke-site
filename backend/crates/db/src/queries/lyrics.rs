//! Query functions for the `lyrics` table.
//!
//! Lyrics rows are shared: multiple songs or performances can point to the same
//! `lyrics_id`.

use sqlx::{Executor, MySql, MySqlConnection};
use uuid::Uuid;

use crate::error::DbError;
use crate::models::lyrics::{Lyrics, NewLyrics};

type Result<T> = std::result::Result<T, DbError>;

/// Fetches a lyrics row by ID.
pub async fn get_by_id(
    executor: impl Executor<'_, Database = MySql>,
    id: Uuid,
) -> Result<Option<Lyrics>> {
    sqlx::query_as::<_, Lyrics>("SELECT id, content FROM lyrics WHERE id = ?")
        .bind(id)
        .fetch_optional(executor)
        .await
        .map_err(DbError::from)
}

/// Inserts a new lyrics row and returns it.
pub async fn create(conn: &mut MySqlConnection, new: &NewLyrics) -> Result<Lyrics> {
    sqlx::query_as::<_, Lyrics>("INSERT INTO lyrics (content) VALUES (?) RETURNING id, content")
        .bind(&new.content)
        .fetch_one(conn)
        .await
        .map_err(DbError::from)
}

/// Updates the content of an existing lyrics row in place.
///
/// Because lyrics rows are shared, this change is visible to every song and
/// performance that references this ID.
pub async fn update(
    executor: impl Executor<'_, Database = MySql>,
    id: Uuid,
    content: &str,
) -> Result<()> {
    sqlx::query("UPDATE lyrics SET content = ? WHERE id = ?")
        .bind(content)
        .bind(id)
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(DbError::from)
}

/// Deletes a lyrics row by ID. Returns `true` if a row was deleted.
pub async fn delete(executor: impl Executor<'_, Database = MySql>, id: Uuid) -> Result<bool> {
    sqlx::query("DELETE FROM lyrics WHERE id = ?")
        .bind(id)
        .execute(executor)
        .await
        .map(|r| r.rows_affected() > 0)
        .map_err(DbError::from)
}

/// Returns the total number of songs and performances that reference this lyrics record.
///
/// Check this before deleting: only delete the row when the count reaches zero.
pub async fn reference_count(
    executor: impl Executor<'_, Database = MySql>,
    id: Uuid,
) -> Result<u64> {
    let count: i64 = sqlx::query_scalar(
        "SELECT \
             (SELECT COUNT(*) FROM songs WHERE lyrics_id = ?) + \
             (SELECT COUNT(*) FROM performances WHERE lyrics_id = ?)",
    )
    .bind(id)
    .bind(id)
    .fetch_one(executor)
    .await
    .map_err(DbError::from)?;
    Ok(count as u64)
}
