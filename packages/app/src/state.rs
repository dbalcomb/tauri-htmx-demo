use std::sync::{Arc, Mutex};

use axum::Router;
use backend::routes::router;

pub struct AppState {
    pub router: Arc<Mutex<Router>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            router: Arc::new(Mutex::new(router())),
        }
    }
}
