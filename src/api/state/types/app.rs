use axum::extract::FromRef;
use deadpool::managed::Pool;
use deadpool_memcached::Manager;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::{
    api::state::types::{jwt_issuer::JwtIssuer, oauth_client::OAuthProviderClient},
    config::types::SessionSecurityConfig,
};

#[derive(Clone)]
pub struct AppState {
    pub oauth_provider_state: Arc<OAuthProviderClient>,
    pub postgres_state: Arc<DatabaseConnection>,
    pub memcached_state: Arc<Pool<Manager>>,
    pub jwt_issuer: Arc<JwtIssuer>,
    pub session_config: Arc<SessionSecurityConfig>,
}

impl FromRef<AppState> for Arc<OAuthProviderClient> {
    fn from_ref(input: &AppState) -> Self {
        input.oauth_provider_state.clone()
    }
}

impl FromRef<AppState> for Arc<DatabaseConnection> {
    fn from_ref(input: &AppState) -> Self {
        input.postgres_state.clone()
    }
}

impl FromRef<AppState> for Arc<Pool<Manager>> {
    fn from_ref(input: &AppState) -> Self {
        input.memcached_state.clone()
    }
}

impl FromRef<AppState> for Arc<JwtIssuer> {
    fn from_ref(input: &AppState) -> Self {
        input.jwt_issuer.clone()
    }
}

impl FromRef<AppState> for Arc<SessionSecurityConfig> {
    fn from_ref(input: &AppState) -> Self {
        input.session_config.clone()
    }
}
