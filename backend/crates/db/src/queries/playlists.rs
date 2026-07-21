//! Query functions for the `playlists` table and its `playlist_performances` join table.

use sqlx::{Executor, MySql, MySqlConnection};
use uuid::Uuid;

use crate::error::DbError;
use crate::models::playlist::{NewPlaylist, Playlist, UpdatePlaylist};

type Result<T> = std::result::Result<T, DbError>;

/// Fetches a playlist by ID.
pub async fn get_by_id(
    executor: impl Executor<'_, Database = MySql>,
    id: Uuid,
) -> Result<Option<Playlist>> {
    sqlx::query_as::<_, Playlist>(
        "SELECT id, title, description, kind, created_by FROM playlists WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(executor)
    .await
    .map_err(DbError::from)
}

/// Returns all playlists ordered by ID.
pub async fn list(executor: impl Executor<'_, Database = MySql>) -> Result<Vec<Playlist>> {
    sqlx::query_as::<_, Playlist>(
        "SELECT id, title, description, kind, created_by FROM playlists ORDER BY id",
    )
    .fetch_all(executor)
    .await
    .map_err(DbError::from)
}

/// Returns all playlists created by a specific user.
pub async fn list_by_user(
    executor: impl Executor<'_, Database = MySql>,
    user_id: Uuid,
) -> Result<Vec<Playlist>> {
    sqlx::query_as::<_, Playlist>(
        "SELECT id, title, description, kind, created_by FROM playlists \
         WHERE created_by = ? ORDER BY id",
    )
    .bind(user_id)
    .fetch_all(executor)
    .await
    .map_err(DbError::from)
}

/// Inserts a new playlist and returns the created row.
pub async fn create(conn: &mut MySqlConnection, new: &NewPlaylist) -> Result<Playlist> {
    sqlx::query_as::<_, Playlist>(
        "INSERT INTO playlists (title, description, kind, created_by) VALUES (?, ?, ?, ?) \
         RETURNING id, title, description, kind, created_by",
    )
    .bind(&new.title)
    .bind(&new.description)
    .bind(&new.kind)
    .bind(new.created_by)
    .fetch_one(conn)
    .await
    .map_err(DbError::from)
}

/// Updates a playlist's mutable fields. Returns `None` if the ID does not exist.
pub async fn update(
    conn: &mut MySqlConnection,
    id: Uuid,
    upd: &UpdatePlaylist,
) -> Result<Option<Playlist>> {
    sqlx::query_as::<_, Playlist>(
        "UPDATE playlists SET title = ?, description = ?, kind = ? WHERE id = ? \
         RETURNING id, title, description, kind, created_by",
    )
    .bind(&upd.title)
    .bind(&upd.description)
    .bind(&upd.kind)
    .bind(id)
    .fetch_optional(conn)
    .await
    .map_err(DbError::from)
}

/// Deletes a playlist by ID. Returns `true` if a row was deleted.
pub async fn delete(executor: impl Executor<'_, Database = MySql>, id: Uuid) -> Result<bool> {
    sqlx::query("DELETE FROM playlists WHERE id = ?")
        .bind(id)
        .execute(executor)
        .await
        .map(|r| r.rows_affected() > 0)
        .map_err(DbError::from)
}

/// Returns the performance IDs in a playlist, ordered by `sort_order`.
pub async fn get_performance_ids(
    executor: impl Executor<'_, Database = MySql>,
    playlist_id: Uuid,
) -> Result<Vec<Uuid>> {
    sqlx::query_scalar::<_, Uuid>(
        "SELECT performance_id FROM playlist_performances \
         WHERE playlist_id = ? ORDER BY sort_order",
    )
    .bind(playlist_id)
    .fetch_all(executor)
    .await
    .map_err(DbError::from)
}

/// Replaces the full ordered set of performances in a playlist.
///
/// The position in `performance_ids` becomes the `sort_order` value.
/// Must be called within a caller provided transaction for atomicity.
pub async fn set_performances(
    conn: &mut MySqlConnection,
    playlist_id: Uuid,
    performance_ids: &[Uuid],
) -> Result<()> {
    sqlx::query("DELETE FROM playlist_performances WHERE playlist_id = ?")
        .bind(playlist_id)
        .execute(&mut *conn)
        .await
        .map_err(DbError::from)?;
    for (pos, &performance_id) in performance_ids.iter().enumerate() {
        sqlx::query(
            "INSERT INTO playlist_performances (playlist_id, performance_id, sort_order) \
             VALUES (?, ?, ?)",
        )
        .bind(playlist_id)
        .bind(performance_id)
        .bind(pos as u32)
        .execute(&mut *conn)
        .await
        .map_err(DbError::from)?;
    }
    Ok(())
}

/// Appends a performance to the end of a playlist. No ops if already present.
///
/// The `sort_order` is set to `MAX(sort_order) + 1`, defaulting to `0` for an
/// empty playlist.
pub async fn add_performance(
    executor: impl Executor<'_, Database = MySql>,
    playlist_id: Uuid,
    performance_id: Uuid,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO playlist_performances (playlist_id, performance_id, sort_order) \
         VALUES (?, ?, COALESCE(\
             (SELECT MAX(sort_order) + 1 FROM playlist_performances WHERE playlist_id = ?), 0)) \
         ON DUPLICATE KEY UPDATE playlist_id = playlist_id",
    )
    .bind(playlist_id)
    .bind(performance_id)
    .bind(playlist_id)
    .execute(executor)
    .await
    .map(|_| ())
    .map_err(DbError::from)
}

/// Removes a single performance from a playlist.
pub async fn remove_performance(
    executor: impl Executor<'_, Database = MySql>,
    playlist_id: Uuid,
    performance_id: Uuid,
) -> Result<()> {
    sqlx::query("DELETE FROM playlist_performances WHERE playlist_id = ? AND performance_id = ?")
        .bind(playlist_id)
        .bind(performance_id)
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(DbError::from)
}
