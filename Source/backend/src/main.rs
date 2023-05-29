mod domain;
mod db;
mod http_api;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let db = db::DbConnnection::connect().await?;
    http_api::host(db).await?;

    Ok(())
}
