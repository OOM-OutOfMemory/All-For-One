use axum::Router;

use crate::api::state::types::app::AppState;

mod jwks;
mod oauth;

pub async fn router(app_state: AppState) -> Router {
    Router::new().nest("/oauth", oauth::router(app_state.clone()).await)
}
