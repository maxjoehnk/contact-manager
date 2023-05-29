use std::cell::RefCell;
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::process::Output;
use std::sync::{Arc, Weak};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Mutex;

mod entities;
pub mod repositories;

#[derive(Clone, Debug)]
pub struct DbConnnection {
    pool: PgPool,
}

type InnerDbContext = Mutex<sqlx::Transaction<'static, sqlx::Postgres>>;

#[derive(Clone)]
#[repr(transparent)]
pub struct DbContext(Arc<InnerDbContext>);

pub struct WeakDbContext(Weak<InnerDbContext>);

impl DbContext {
    pub async fn new(connection: &DbConnnection) -> color_eyre::Result<Self> {
        let transaction = connection.pool.begin().await?;
        let transaction = Self(Arc::new(Mutex::new(transaction)));

        Ok(transaction)
    }

    pub async fn commit(mut self) -> color_eyre::Result<()> {
        let transaction = Arc::try_unwrap(self.0).unwrap();
        let transaction = transaction.into_inner();
        transaction.commit().await?;
        Ok(())
    }

    pub fn downgrade(&self) -> WeakDbContext {
        WeakDbContext(Arc::downgrade(&self.0))
    }
}

impl WeakDbContext {
    pub fn upgrade(&self) -> DbContext {
        DbContext(self.0.upgrade().unwrap())
    }
}

impl Deref for DbContext {
    type Target = sqlx::Transaction<'static, sqlx::Postgres>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl DerefMut for DbContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::get_mut(&mut self.0).unwrap()
    }
}

impl DbConnnection {
    pub async fn connect() -> color_eyre::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect("postgres://contact-manager:db@localhost/contact-manager")
            .await?;

        sqlx::migrate!().run(&pool).await?;

        Ok(DbConnnection { pool })
    }
}
