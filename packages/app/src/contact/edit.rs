use askama::Template;
use axum::extract::State;

use crate::router::RouterState;

use super::Contact;

pub async fn get(State(state): State<RouterState>) -> Result<ContactForm, String> {
    let form = ContactForm {
        contact: state.contact.lock().unwrap().clone(),
    };

    Ok(form)
}

#[derive(Template)]
#[template(path = "contact-form.html")]
pub struct ContactForm {
    pub contact: Contact,
}
