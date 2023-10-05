use std::sync::{Arc, OnceLock};

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pool: Arc<OnceLock<SqlitePool>>,
}

impl AppState {
    /// Gets the database pool, initialising it if it has not been set.
    ///
    /// The `sqlx` database pool must be initialised in a Tokio runtime context
    /// and so cannot be constructed in the application state even if using the
    /// lazy connection option. This lazily constructs the pool when it is first
    /// accessed which should be from within an async Tauri command that uses
    /// the Tokio runtime.
    ///
    /// An alternative solution would be to pass an existing Tokio runtime to
    /// Tauri using [this](https://docs.rs/tauri/latest/tauri/async_runtime/fn.set.html)
    /// documentation. However, that would assume that the database is known at
    /// application startup and would not support the ability to select a target
    /// database from the frontend.
    pub fn pool(&self) -> &SqlitePool {
        self.pool.get_or_init(|| {
            SqlitePool::connect_lazy_with(
                SqliteConnectOptions::new()
                    .filename("../../sqlite.db")
                    .create_if_missing(false),
            )
        })
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            pool: Arc::new(OnceLock::new()),
        }
    }
}
