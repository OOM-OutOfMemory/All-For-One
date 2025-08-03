use std::sync::Arc;

use axum::{Router, extract::State, routing::get};
use sea_orm::{ConnectionTrait, EntityTrait, Statement};
use tracing::info;

use crate::api::{state::types::app::AppState, v1};

pub async fn make_server_route(app_state: AppState) -> Router {
    Router::new().nest(
        "/api",
        Router::new()
            .route("/heartbeat", get(heartbeat))
            .with_state(app_state.clone())
            .nest("/v1", v1::router(app_state.clone()).await),
    )
}

async fn heartbeat() -> &'static str {
    "My OIDC server is running!"
}
