use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;

use crate::db::{DbConnnection, DbContext, WeakDbContext};
use crate::http_api;

// pub struct TransactionLayer(DbConnnection);
//
// impl TransactionLayer {
//     pub fn new(db: DbConnnection) -> Self {
//         Self(db)
//     }
// }
//
// impl<S> Layer<S> for TransactionLayer {
//     type Service = TransactionService<S>;
//
//     fn layer(&self, inner: S) -> Self::Service {
//         TransactionService {
//             inner,
//             db: self.0.clone(),
//         }
//     }
// }
//
// struct TransactionService<S> {
//     inner: S,
//     db: DbConnnection,
// }
//
// impl<S, B> Service<Request<B>> for TransactionService<S>
// where S: Service<Request<B>> {
//     type Response = S::Response;
//     type Error = S::Error;
//     type Future = S::Future;
//
//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         todo!()
//     }
//
//     fn call(&mut self, req: Request<B>) -> Self::Future {
//         todo!()
//     }
// }

pub async fn with_db<B>(State(db): State<DbConnnection>, mut req: Request<B>, next: Next<B>) -> http_api::Result<Response> {
    let context = DbContext::new(&db).await?;

    req.extensions_mut().insert(context.downgrade());

    let res = next.run(req).await;
    if res.status().is_client_error() || res.status().is_server_error() {
        return Ok(res);
    }

    context.commit().await?;

    Ok(res)
}

#[async_trait]
impl<S> FromRequestParts<S> for DbContext
    where S: Send + Sync {
    type Rejection = http_api::ServiceError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let db_context = parts.extensions.get::<WeakDbContext>().unwrap();

        Ok(db_context.upgrade())
    }
}
