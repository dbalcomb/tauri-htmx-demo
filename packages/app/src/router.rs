use std::sync::{Arc, Mutex};

use axum::routing::{get, put};
use axum::Router;

use crate::contact::Contact;

#[derive(Clone)]
pub struct RouterState {
    pub contact: Arc<Mutex<Contact>>,
}

impl Default for RouterState {
    fn default() -> Self {
        Self {
            contact: Arc::new(Mutex::new(Contact::default())),
        }
    }
}

pub fn create_router() -> Router {
    Router::new()
        .route("/contact/1", get(crate::contact::get))
        .route("/contact/1", put(crate::contact::update))
        .route("/contact/1/edit", get(crate::contact::edit::get))
        .with_state(RouterState::default())
}
