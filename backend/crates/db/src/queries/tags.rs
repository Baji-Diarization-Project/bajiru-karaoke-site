//! Query functions for the `tags` table.

use sqlx::{Executor, MySql, MySqlConnection};
use uuid::Uuid;

use crate::error::DbError;
use crate::models::tag::{NewTag, Tag};

type Result<T> = std::result::Result<T, DbError>;

/// Fetches a tag by ID.
pub async fn get_by_id(
    executor: impl Executor<'_, Database = MySql>,
    id: Uuid,
) -> Result<Option<Tag>> {
    sqlx::query_as::<_, Tag>("SELECT id, name FROM tags WHERE id = ?")
        .bind(id)
        .fetch_optional(executor)
        .await
        .map_err(DbError::from)
}

/// Returns all tags ordered by name.
pub async fn list(executor: impl Executor<'_, Database = MySql>) -> Result<Vec<Tag>> {
    sqlx::query_as::<_, Tag>("SELECT id, name FROM tags ORDER BY name")
        .fetch_all(executor)
        .await
        .map_err(DbError::from)
}

/// Returns the tag with the given name, creating it if it does not exist.
pub async fn get_or_create(conn: &mut MySqlConnection, new: &NewTag) -> Result<Tag> {
    sqlx::query("INSERT INTO tags (name) VALUES (?) ON DUPLICATE KEY UPDATE id = id")
        .bind(&new.name)
        .execute(&mut *conn)
        .await
        .map_err(DbError::from)?;
    sqlx::query_as::<_, Tag>("SELECT id, name FROM tags WHERE name = ?")
        .bind(&new.name)
        .fetch_one(&mut *conn)
        .await
        .map_err(DbError::from)
}

/// Deletes a tag by ID. Returns `true` if a row was deleted.
pub async fn delete(executor: impl Executor<'_, Database = MySql>, id: Uuid) -> Result<bool> {
    sqlx::query("DELETE FROM tags WHERE id = ?")
        .bind(id)
        .execute(executor)
        .await
        .map(|r| r.rows_affected() > 0)
        .map_err(DbError::from)
}
