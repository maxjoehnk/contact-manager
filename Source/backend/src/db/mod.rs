use std::fmt::{Debug};
use std::sync::{Arc, Weak};
use sqlx::{Describe, Either, Error, Execute, Executor, PgPool};
use sqlx::database::HasStatement;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::{Mutex,};
use futures::future::{BoxFuture, FutureExt};
use futures::stream::{BoxStream, StreamExt};

pub mod repositories;

#[derive(Clone, Debug)]
pub struct DbConnnection {
    pool: PgPool,
}

type InnerDbContext = Mutex<sqlx::Transaction<'static, sqlx::Postgres>>;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct DbContext(Arc<InnerDbContext>);

pub struct WeakDbContext(Weak<InnerDbContext>);

impl DbContext {
    pub async fn new(connection: &DbConnnection) -> color_eyre::Result<Self> {
        let transaction = connection.pool.begin().await?;
        let transaction = Self(Arc::new(Mutex::new(transaction)));

        Ok(transaction)
    }

    pub async fn commit(self) -> color_eyre::Result<()> {
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

impl<'c> Executor<'c> for &'c DbContext {
    type Database = sqlx::Postgres;

    fn fetch_many<'e, 'q: 'e, E: 'q>(self, query: E) -> BoxStream<
        'e,
        Result<
            Either<<Self::Database as sqlx::Database>::QueryResult, <Self::Database as sqlx::Database>::Row>,
            sqlx::Error,
        >,
    > where
        'c: 'e,
        E: Execute<'q, Self::Database>{
        // TODO: this is not optimal as it will capture all the rows in memory before building a new stream
        let items = async {
            let mut guard = self.0.lock().await;
            guard.fetch_many(query).collect::<Vec<_>>().await
        };
        items
            .into_stream()
            .flat_map(|items| futures::stream::iter(items))
            .boxed()
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(self, query: E) -> BoxFuture<'e, Result<Option<<Self::Database as sqlx::Database>::Row>, Error>>
        where 'c: 'e, E: Execute<'q, Self::Database>
    {
        async {
            let mut guard = self.0.lock().await;
            guard.fetch_optional(query).await
        }.boxed()
    }

    fn prepare_with<'e, 'q: 'e>(self, sql: &'q str, parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo]) -> BoxFuture<'e, Result<<Self::Database as HasStatement<'q>>::Statement, Error>>
        where 'c: 'e {
        async {
            let mut guard = self.0.lock().await;
            guard.prepare_with(sql, parameters).await
        }.boxed()
    }

    fn describe<'e, 'q: 'e>(self, sql: &'q str) -> BoxFuture<'e, Result<Describe<Self::Database>, Error>> where 'c: 'e {
        async {
            let mut guard = self.0.lock().await;
            guard.describe(sql).await
        }.boxed()
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
