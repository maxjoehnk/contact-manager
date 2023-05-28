use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

#[derive(Clone, Debug)]
pub struct DbConnnection {
    pool: PgPool,
}

pub async fn connect_db() -> color_eyre::Result<DbConnnection> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect("postgres://postgres:postgres@localhost/contact-manager")
        .await?;

    sqlx::migrate!("migrations").run(&pool).await?;

    Ok(DbConnnection { pool })
}
