pub(crate) mod session_cleanup;

/// Spawns all background tasks. Intended to be called once at startup.
pub fn spawn_background_tasks(pool: db::MySqlPool) {
    tokio::spawn(session_cleanup::run(pool));
}
