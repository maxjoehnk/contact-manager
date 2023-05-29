use std::net::SocketAddr;

use axum::{Json, Router};
use axum::http::StatusCode;
use axum::middleware::from_fn_with_state;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use serde::Deserialize;

use crate::db::DbConnnection;
use crate::domain::{Contact, ContactId, EmailContact, Person};
use crate::events::{ContactEvent, ContactEventType};

mod contacts;
mod middleware;

pub async fn host(db: DbConnnection) -> color_eyre::Result<()> {
    let v1_api = Router::new()
        .nest("/contacts", contacts::router())
        .route_layer(from_fn_with_state(db.clone(), middleware::with_db));
    let app = Router::new().nest("/api/v1", v1_api);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

pub struct ServiceError(color_eyre::Report);

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            self.0.to_string()
        )
            .into_response()
    }
}

impl<E> From<E> for ServiceError where E: Into<color_eyre::Report> {
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

pub type Result<T> = core::result::Result<T, ServiceError>;
