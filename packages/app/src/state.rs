use std::sync::{Arc, Mutex};

use axum::Router;

use crate::router::create_router;

pub struct AppState {
    pub router: Arc<Mutex<Router>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            router: Arc::new(Mutex::new(create_router())),
        }
    }
}
