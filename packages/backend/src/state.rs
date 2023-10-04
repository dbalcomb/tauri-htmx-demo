use std::sync::{Arc, Mutex};

use crate::contact::Contact;

#[derive(Clone)]
pub struct AppState {
    pub contact: Arc<Mutex<Contact>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            contact: Arc::new(Mutex::new(Contact::default())),
        }
    }
}
