pub mod edit;

use askama::Template;
use axum::extract::{Form, State};
use serde::Deserialize;

use crate::state::AppState;

pub async fn get(State(state): State<AppState>) -> Result<Contact, String> {
    Ok(state.contact.lock().unwrap().clone())
}

pub async fn update(
    State(state): State<AppState>,
    Form(contact): Form<Contact>,
) -> Result<Contact, String> {
    *state.contact.lock().unwrap() = contact.clone();

    Ok(contact)
}

#[derive(Clone, Debug, Deserialize, Template)]
#[template(path = "contact.html")]
#[serde(rename_all = "snake_case")]
pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl Default for Contact {
    fn default() -> Self {
        Self {
            first_name: String::from("Joe"),
            last_name: String::from("Bloggs"),
            email: String::from("joe.bloggs@example.com"),
        }
    }
}
