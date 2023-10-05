pub mod contacts;
pub mod error;

use axum::routing::{delete, get, post, put};
use axum::Router;

use crate::state::AppState;

pub fn router() -> Router {
    Router::new()
        .route("/contacts", get(self::contacts::list))
        .route("/contacts", post(self::contacts::create))
        .route("/contacts/new", get(self::contacts::new))
        .route("/contacts/:id", get(self::contacts::view))
        .route("/contacts/:id", put(self::contacts::update))
        .route("/contacts/:id", delete(self::contacts::delete))
        .route("/contacts/:id/edit", get(self::contacts::edit))
        .route("/contacts/:id/delete", get(self::contacts::remove))
        .with_state(AppState::default())
}
