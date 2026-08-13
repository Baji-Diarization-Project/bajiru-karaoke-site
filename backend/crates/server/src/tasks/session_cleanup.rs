//! Background task that periodically removes expired session rows.

use std::time::Duration;

use db::{MySqlPool, queries};
use tokio::time;

const INTERVAL: Duration = Duration::from_secs(60 * 60);

/// Runs the expired-session sweep on a fixed interval.
///
/// Intended to be spawned with [`tokio::spawn`] at startup, runs until the
/// process exits.
pub async fn run(pool: MySqlPool) {
    let mut interval = time::interval(INTERVAL);
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
    loop {
        interval.tick().await;
        match queries::sessions::delete_expired(&pool).await {
            Ok(n) if n > 0 => tracing::info!(deleted = n, "expired sessions removed"),
            Ok(_) => {}
            Err(e) => tracing::error!(error = %e, "session cleanup failed"),
        }
    }
}
