use std::net::SocketAddr;
use axum::{Json, Router};
use axum::routing::get;
use crate::db::DbConnnection;
use crate::domain::Contact;

mod domain;
mod events;
mod db;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let db = db::connect_db().await?;

    let app = Router::new()
        .route("/contacts", get(list_contacts))
        .with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}


async fn list_contacts() -> Json<Vec<Contact>> {
    vec![].into()
}
