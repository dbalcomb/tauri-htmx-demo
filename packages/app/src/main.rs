// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

use askama::Template;
use serde::Deserialize;
use tauri::{State, Window};

fn main() {
    tauri::Builder::default()
        .manage(Data {
            contact: Mutex::new(Contact::default()),
        })
        .invoke_handler(tauri::generate_handler![window_did_finish_loading, htmx])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn window_did_finish_loading(window: Window) -> Result<(), tauri::Error> {
    println!("Window did finish loading.");
    println!();

    if !window.is_visible()? {
        window.show()?;
    }

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
async fn htmx(
    method: String,
    url: PathBuf,
    body: Option<String>,
    headers: HashMap<String, String>,
    state: State<'_, Data>,
) -> Result<String, String> {
    println!("HTMX:");
    println!("Path: {}", url.display());
    println!("Method: {method}");
    println!("Body: {}", body.as_deref().unwrap_or_default());
    println!("Headers: {headers:?}");
    println!();

    println!("Contact: {:?}", state.contact.lock().unwrap());
    println!();

    match (&*method, url.as_os_str().to_str()) {
        ("GET", Some("/contact/1")) => state
            .contact
            .lock()
            .unwrap()
            .render()
            .map_err(|err| err.to_string()),
        ("GET", Some("/contact/1/edit")) => ContactForm {
            contact: state.contact.lock().unwrap().clone(),
        }
        .render()
        .map_err(|err| err.to_string()),
        ("PUT", Some("/contact/1")) => {
            let contact: Contact =
                serde_qs::from_str(&body.unwrap()).map_err(|err| err.to_string())?;
            *state.contact.lock().unwrap() = contact;
            state
                .contact
                .lock()
                .unwrap()
                .render()
                .map_err(|err| err.to_string())
        }
        _ => Err(String::from("Invalid path.")),
    }
}

pub struct Data {
    contact: Mutex<Contact>,
}

#[derive(Clone, Debug, Deserialize, Template)]
#[template(path = "contact.html")]
#[serde(rename_all = "snake_case")]
struct Contact {
    first_name: String,
    last_name: String,
    email: String,
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

#[derive(Template)]
#[template(path = "contact-form.html")]
struct ContactForm {
    contact: Contact,
}
