use axum::body::Body;
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::get;
use crate::db::DbContext;
use crate::domain::{CreateDistributionList, DistributionList, DistributionListId};
use crate::domain::services::DistributionListsService;
use crate::http_api;

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S, Body> {
    Router::new()
        .route("/", get(list_distribution_lists).post(create_distribution_list))
}

async fn list_distribution_lists(db: DbContext) -> http_api::Result<Json<Vec<DistributionList>>> {
    let distribution_lists = DistributionListsService.list(&db).await?;

    Ok(Json(distribution_lists))
}

async fn create_distribution_list(db: DbContext, Json(distribution_list): Json<CreateDistributionList>) -> http_api::Result<(StatusCode, Json<DistributionListId>)> {
    let distribution_list_id = DistributionListsService.create(&db, distribution_list).await?;

    Ok((StatusCode::CREATED, Json(distribution_list_id)))
}
