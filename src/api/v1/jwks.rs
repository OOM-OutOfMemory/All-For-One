use std::sync::Arc;

use anyhow::Result;
use axum::{
    Json, Router,
    extract::State,
    response::{IntoResponse, Response},
    routing::get,
};

use crate::{
    api::state::types::{app::AppState, jwt_issuer::JwtIssuer},
    utils::error::AllForOneError,
};

async fn jwks(State(jwt_issuer): State<Arc<JwtIssuer>>) -> Result<Response, AllForOneError> {
    let jwks = jwt_issuer.jwks()?;
    Ok((Json(jwks)).into_response())
}

pub async fn router(app_state: AppState) -> Router {
    axum::Router::new()
        .route("/", get(jwks))
        .with_state(app_state)
}
