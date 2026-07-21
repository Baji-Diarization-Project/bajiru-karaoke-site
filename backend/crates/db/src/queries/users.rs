//! Query functions for the `users` table.

use sqlx::{Executor, MySql, MySqlConnection};
use uuid::Uuid;

use crate::error::DbError;
use crate::models::user::{NewUser, UpdateUser, User};

type Result<T> = std::result::Result<T, DbError>;

/// Fetches a user by primary key.
pub async fn get_by_id(
    executor: impl Executor<'_, Database = MySql>,
    id: Uuid,
) -> Result<Option<User>> {
    sqlx::query_as::<_, User>("SELECT id, username, twitch_id, discord_id FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(executor)
        .await
        .map_err(DbError::from)
}

/// Fetches a user by Twitch provider ID.
pub async fn get_by_twitch_id(
    executor: impl Executor<'_, Database = MySql>,
    twitch_id: u64,
) -> Result<Option<User>> {
    sqlx::query_as::<_, User>(
        "SELECT id, username, twitch_id, discord_id FROM users WHERE twitch_id = ?",
    )
    .bind(twitch_id)
    .fetch_optional(executor)
    .await
    .map_err(DbError::from)
}

/// Fetches a user by Discord provider ID.
pub async fn get_by_discord_id(
    executor: impl Executor<'_, Database = MySql>,
    discord_id: u64,
) -> Result<Option<User>> {
    sqlx::query_as::<_, User>(
        "SELECT id, username, twitch_id, discord_id FROM users WHERE discord_id = ?",
    )
    .bind(discord_id)
    .fetch_optional(executor)
    .await
    .map_err(DbError::from)
}

/// Fetches a user by username (case insensitive via DB collation).
pub async fn get_by_username(
    executor: impl Executor<'_, Database = MySql>,
    username: &str,
) -> Result<Option<User>> {
    sqlx::query_as::<_, User>(
        "SELECT id, username, twitch_id, discord_id FROM users WHERE username = ?",
    )
    .bind(username)
    .fetch_optional(executor)
    .await
    .map_err(DbError::from)
}

/// Returns all users ordered by ID.
pub async fn list(executor: impl Executor<'_, Database = MySql>) -> Result<Vec<User>> {
    sqlx::query_as::<_, User>("SELECT id, username, twitch_id, discord_id FROM users ORDER BY id")
        .fetch_all(executor)
        .await
        .map_err(DbError::from)
}

/// Inserts a new user and returns the created row.
///
/// # Errors
///
/// Returns [`DbError::Conflict`] if the username is already taken.
pub async fn create(conn: &mut MySqlConnection, new: &NewUser) -> Result<User> {
    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, twitch_id, discord_id) VALUES (?, ?, ?) \
         RETURNING id, username, twitch_id, discord_id",
    )
    .bind(&new.username)
    .bind(new.twitch_id)
    .bind(new.discord_id)
    .fetch_one(conn)
    .await;

    match result {
        Ok(user) => Ok(user),
        Err(sqlx::Error::Database(e)) if e.is_unique_violation() => Err(DbError::Conflict),
        Err(e) => Err(DbError::Sqlx(e)),
    }
}

/// Inserts a user keyed on `twitch_id`, updating `username` on conflict.
///
/// Used on every successful Twitch OAuth login so the username stays in sync
/// with the user's current Twitch display name.
pub async fn upsert_by_twitch(conn: &mut MySqlConnection, new: &NewUser) -> Result<User> {
    sqlx::query(
        "INSERT INTO users (username, twitch_id, discord_id) VALUES (?, ?, ?) \
         ON DUPLICATE KEY UPDATE username = VALUES(username)",
    )
    .bind(&new.username)
    .bind(new.twitch_id)
    .bind(new.discord_id)
    .execute(&mut *conn)
    .await
    .map_err(DbError::from)?;
    get_by_twitch_id(&mut *conn, new.twitch_id.ok_or(DbError::NotFound)?)
        .await?
        .ok_or(DbError::NotFound)
}

/// Inserts a user keyed on `discord_id`, updating `username` on conflict.
///
/// Used on every successful Discord OAuth login so the username stays in sync
/// with the user's current Discord username.
pub async fn upsert_by_discord(conn: &mut MySqlConnection, new: &NewUser) -> Result<User> {
    sqlx::query(
        "INSERT INTO users (username, twitch_id, discord_id) VALUES (?, ?, ?) \
         ON DUPLICATE KEY UPDATE username = VALUES(username)",
    )
    .bind(&new.username)
    .bind(new.twitch_id)
    .bind(new.discord_id)
    .execute(&mut *conn)
    .await
    .map_err(DbError::from)?;
    get_by_discord_id(&mut *conn, new.discord_id.ok_or(DbError::NotFound)?)
        .await?
        .ok_or(DbError::NotFound)
}

/// Replaces all mutable fields on a user. Returns `None` if the ID does not exist.
pub async fn update(
    conn: &mut MySqlConnection,
    id: Uuid,
    upd: &UpdateUser,
) -> Result<Option<User>> {
    sqlx::query_as::<_, User>(
        "UPDATE users SET username = ?, twitch_id = ?, discord_id = ? WHERE id = ? \
         RETURNING id, username, twitch_id, discord_id",
    )
    .bind(&upd.username)
    .bind(upd.twitch_id)
    .bind(upd.discord_id)
    .bind(id)
    .fetch_optional(conn)
    .await
    .map_err(DbError::from)
}

/// Deletes a user by ID. Returns `true` if a row was deleted.
pub async fn delete(executor: impl Executor<'_, Database = MySql>, id: Uuid) -> Result<bool> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(executor)
        .await
        .map(|r| r.rows_affected() > 0)
        .map_err(DbError::from)
}

/// Returns the IDs of all songs the user has favorited.
pub async fn get_favorite_song_ids(
    executor: impl Executor<'_, Database = MySql>,
    user_id: Uuid,
) -> Result<Vec<Uuid>> {
    sqlx::query_scalar::<_, Uuid>("SELECT song_id FROM user_favorite_songs WHERE user_id = ?")
        .bind(user_id)
        .fetch_all(executor)
        .await
        .map_err(DbError::from)
}

/// Adds a song to the user's favorites. No ops if already favorited.
pub async fn add_favorite_song(
    executor: impl Executor<'_, Database = MySql>,
    user_id: Uuid,
    song_id: Uuid,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO user_favorite_songs (user_id, song_id) VALUES (?, ?) \
         ON DUPLICATE KEY UPDATE user_id = user_id",
    )
    .bind(user_id)
    .bind(song_id)
    .execute(executor)
    .await
    .map(|_| ())
    .map_err(DbError::from)
}

/// Removes a song from the user's favorites.
pub async fn remove_favorite_song(
    executor: impl Executor<'_, Database = MySql>,
    user_id: Uuid,
    song_id: Uuid,
) -> Result<()> {
    sqlx::query("DELETE FROM user_favorite_songs WHERE user_id = ? AND song_id = ?")
        .bind(user_id)
        .bind(song_id)
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(DbError::from)
}
