use std::sync::Arc;
use axum::{Extension, Json, middleware, Router};
use axum::body::{Body, HttpBody};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use serde::Deserialize;
use crate::db::{DbConnnection, DbContext};
use crate::db::repositories::ContactRepository;

use crate::domain::*;
use crate::events::*;
use crate::http_api;

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S, Body> {
    Router::new()
        .route("/", get(list_contacts))
        .route("/persons", get(list_persons).post(create_person))
        // .route("/organization", post(create_organization))
}

async fn list_contacts(mut db: DbContext) -> http_api::Result<Json<Vec<Contact>>> {
    Ok(Json(vec![]))
}

async fn list_persons(mut db: DbContext) -> http_api::Result<Json<Vec<Person>>> {
    let persons = ContactRepository.list_persons(&mut db).await?;

    Ok(Json(persons))
}

async fn create_person(mut db: DbContext, Json(person): Json<CreatePerson>) -> http_api::Result<(StatusCode, Json<ContactId>)> {
    let contact_id = ContactId::new();
    let event = ContactEvent {
        contact_id,
        event: ContactEventType::PersonAdded {
            first_name: person.first_name.clone(),
            last_name: person.last_name.clone(),
            email: person.email.clone(),
        },
    };

    ContactRepository.create_person(&mut db, Person {
        id: contact_id,
        first_name: person.first_name,
        last_name: person.last_name,
        email: person.email,
    }).await?;

    Ok((
        StatusCode::CREATED,
        Json(contact_id),
    ))
}

#[derive(Debug, Clone, Deserialize)]
struct CreatePerson {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Vec<EmailContact>,
}

async fn create_organization(Json(organization): Json<CreateOrganization>) -> color_eyre::Result<(StatusCode, Json<ContactId>)> {
    let contact_id = ContactId::new();
    let event = ContactEvent {
        contact_id,
        event: ContactEventType::OrganizationAdded {
            name: organization.name,
            email: organization.email,
        },
    };

    Ok((
        StatusCode::CREATED,
        Json(contact_id),
    ))
}

#[derive(Debug, Clone, Deserialize)]
struct CreateOrganization {
    name: String,
    email: Vec<EmailContact>,
}
