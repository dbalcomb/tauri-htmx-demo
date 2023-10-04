use axum::routing::{get, put};
use axum::Router;

use crate::state::AppState;

pub fn router() -> Router {
    Router::new()
        .route("/contact/1", get(crate::contact::get))
        .route("/contact/1", put(crate::contact::update))
        .route("/contact/1/edit", get(crate::contact::edit::get))
        .with_state(AppState::default())
}
