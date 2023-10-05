use askama::Template;
use axum::extract::{Form, Path, State};
use itertools::Itertools;
use serde::Deserialize;

use crate::state::AppState;

use super::error::Error;

pub async fn list(State(state): State<AppState>) -> Result<ContactList, Error> {
    let mut conn = state.pool().acquire().await?;

    let contacts = sqlx::query_as!(Contact, "SELECT * FROM contacts")
        .fetch_all(&mut *conn)
        .await?;

    Ok(ContactList::new(contacts))
}

pub async fn create(
    State(state): State<AppState>,
    Form(contact): Form<NewContact>,
) -> Result<Contact, Error> {
    let mut conn = state.pool().acquire().await?;

    let id: i64 = sqlx::query_scalar!(
        "INSERT INTO contacts (first_name, last_name, email) VALUES ($1, $2, $3) RETURNING id",
        contact.first_name,
        contact.last_name,
        contact.email
    )
    .fetch_one(&mut *conn)
    .await?;

    Ok(Contact::new(id, contact))
}

pub async fn new() -> NewContactForm {
    NewContactForm::default()
}

pub async fn view(State(state): State<AppState>, Path(id): Path<i64>) -> Result<Contact, Error> {
    let mut conn = state.pool().acquire().await?;

    let contact = sqlx::query_as!(Contact, "SELECT * FROM contacts WHERE id = $1", id)
        .fetch_one(&mut *conn)
        .await?;

    Ok(contact)
}

pub async fn edit(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<ContactForm, Error> {
    let mut conn = state.pool().acquire().await?;

    let contact = sqlx::query_as!(Contact, "SELECT * FROM contacts WHERE id = $1", id)
        .fetch_one(&mut *conn)
        .await?;

    Ok(ContactForm { contact })
}

pub async fn update(
    State(state): State<AppState>,
    Form(contact): Form<Contact>,
) -> Result<Contact, Error> {
    let mut conn = state.pool().acquire().await?;

    sqlx::query!(
        "UPDATE contacts SET first_name = $1, last_name = $2, email = $3 where id = $4",
        contact.first_name,
        contact.last_name,
        contact.email,
        contact.id
    )
    .execute(&mut *conn)
    .await?;

    Ok(contact)
}

pub async fn remove(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<ContactDeleteConfirm, Error> {
    let mut conn = state.pool().acquire().await?;

    let contact = sqlx::query_as!(Contact, "SELECT * FROM contacts WHERE id = $1", id)
        .fetch_one(&mut *conn)
        .await?;

    Ok(ContactDeleteConfirm { contact })
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<ContactList, Error> {
    let mut conn = state.pool().acquire().await?;

    sqlx::query!("DELETE FROM contacts WHERE id = $1", id)
        .execute(&mut *conn)
        .await?;

    let contacts = sqlx::query_as!(Contact, "SELECT * FROM contacts")
        .fetch_all(&mut *conn)
        .await?;

    Ok(ContactList::new(contacts))
}

#[derive(Clone, Debug, Deserialize, Template)]
#[template(path = "contacts/view.html")]
#[serde(rename_all = "snake_case")]
pub struct Contact {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl Contact {
    pub fn new(id: i64, contact: NewContact) -> Self {
        Self {
            id,
            first_name: contact.first_name,
            last_name: contact.last_name,
            email: contact.email,
        }
    }
}

#[derive(Template)]
#[template(path = "contacts/edit.html")]
pub struct ContactForm {
    pub contact: Contact,
}

#[derive(Default, Deserialize)]
pub struct NewContact {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Default, Template)]
#[template(path = "contacts/new.html")]
pub struct NewContactForm {
    pub contact: NewContact,
}

#[derive(Template)]
#[template(path = "contacts.html")]
pub struct ContactList {
    pub groups: Vec<ContactListGroup>,
}

impl ContactList {
    pub fn new(contacts: Vec<Contact>) -> Self {
        Self {
            groups: contacts
                .into_iter()
                .sorted_by_key(|contact| (contact.first_name.clone(), contact.last_name.clone()))
                .group_by(|contact| contact.first_name.chars().next().unwrap())
                .into_iter()
                .sorted_by_key(|(label, _)| *label)
                .map(|(label, contacts)| ContactListGroup::new(label, contacts))
                .collect(),
        }
    }
}

pub struct ContactListGroup {
    pub label: String,
    pub contacts: Vec<Contact>,
}

impl ContactListGroup {
    pub fn new(label: impl Into<String>, contacts: impl IntoIterator<Item = Contact>) -> Self {
        Self {
            label: label.into(),
            contacts: contacts.into_iter().collect(),
        }
    }
}

#[derive(Template)]
#[template(path = "contacts/delete.html")]
pub struct ContactDeleteConfirm {
    pub contact: Contact,
}
