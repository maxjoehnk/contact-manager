use axum::{Json, Router};
use axum::body::Body;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{delete, get};
use uuid::Uuid;

use crate::db::DbContext;
use crate::domain::*;
use crate::domain::services::{ContactsService, OrganizationsService, PersonsService};
use crate::http_api;

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S, Body> {
    Router::new()
        .route("/", get(list_contacts))
        .route("/persons", get(list_persons).post(create_person))
        .route("/persons/:contact_id", delete(delete_person))
        .route("/organizations", get(list_organizations).post(create_organization))
        .route("/organizations/:contact_id", delete(delete_organization))
}

async fn list_contacts(db: DbContext) -> http_api::Result<Json<Vec<Contact>>> {
    let contacts = ContactsService.list(&db).await?;

    Ok(Json(contacts))
}

async fn list_persons(db: DbContext) -> http_api::Result<Json<Vec<Person>>> {
    let persons = PersonsService.list(&db).await?;

    Ok(Json(persons))
}

async fn create_person(db: DbContext, Json(person): Json<CreatePerson>) -> http_api::Result<(StatusCode, Json<ContactId>)> {
    let contact_id = PersonsService.create(&db, person).await?;

    Ok((
        StatusCode::CREATED,
        Json(contact_id),
    ))
}

async fn delete_person(db: DbContext, Path(contact_id): Path<Uuid>) -> http_api::Result<StatusCode> {
    PersonsService.delete(&db, contact_id.into()).await?;

    Ok(StatusCode::NO_CONTENT, )
}

async fn list_organizations(db: DbContext) -> http_api::Result<Json<Vec<Organization>>> {
    let organizations = OrganizationsService.list(&db).await?;

    Ok(Json(organizations))
}

async fn create_organization(db: DbContext, Json(organization): Json<CreateOrganization>) -> http_api::Result<(StatusCode, Json<ContactId>)> {
    let contact_id = OrganizationsService.create(&db, organization).await?;

    Ok((
        StatusCode::CREATED,
        Json(contact_id),
    ))
}

async fn delete_organization(db: DbContext, Path(contact_id): Path<Uuid>) -> http_api::Result<StatusCode> {
    OrganizationsService.delete(&db, contact_id.into()).await?;

    Ok(StatusCode::NO_CONTENT, )
}

