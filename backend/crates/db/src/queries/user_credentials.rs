//! Query functions for the `user_credentials` table.

use sqlx::{Executor, MySql};
use uuid::Uuid;

use crate::{error::DbError, models::user_credential::UserCredential};

type Result<T> = std::result::Result<T, DbError>;

/// Stores an argon2 password hash for a user.
///
/// Should be called once immediately after the user row is created.
pub async fn create(
    executor: impl Executor<'_, Database = MySql>,
    user_id: Uuid,
    password_hash: &str,
) -> Result<()> {
    sqlx::query("INSERT INTO user_credentials (user_id, password_hash) VALUES (?, ?)")
        .bind(user_id)
        .bind(password_hash)
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(DbError::from)
}

/// Fetches the credential for a user. Returns `None` for OAuth only accounts.
pub async fn get_by_user_id(
    executor: impl Executor<'_, Database = MySql>,
    user_id: Uuid,
) -> Result<Option<UserCredential>> {
    sqlx::query_as::<_, UserCredential>(
        "SELECT user_id, password_hash FROM user_credentials WHERE user_id = ?",
    )
    .bind(user_id)
    .fetch_optional(executor)
    .await
    .map_err(DbError::from)
}
