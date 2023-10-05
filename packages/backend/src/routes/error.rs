use std::fmt::{self, Display};

use axum::body;
use axum::response::{IntoResponse, Response};

pub struct Error {
    error: Box<dyn std::error::Error>,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        Response::builder()
            .status(500)
            .body(body::boxed(self.to_string()))
            .unwrap()
    }
}

impl<T> From<T> for Error
where
    T: std::error::Error + 'static,
{
    fn from(err: T) -> Self {
        Self {
            error: Box::new(err),
        }
    }
}
