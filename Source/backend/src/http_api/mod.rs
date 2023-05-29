use std::net::SocketAddr;

use axum::{ Router};
use axum::http::StatusCode;
use axum::middleware::from_fn_with_state;
use axum::response::{IntoResponse, Response};

use crate::db::DbConnnection;

mod contacts;
mod middleware;
mod distribution_lists;

pub async fn host(db: DbConnnection) -> color_eyre::Result<()> {
    let v1_api = Router::new()
        .nest("/contacts", contacts::router())
        .nest("/distributionLists", distribution_lists::router())
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
        tracing::error!("{:?}", self.0);
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
