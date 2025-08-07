use std::sync::Arc;

use anyhow::Result;

use crate::{
    api::{
        state::types::{app::AppState, jwt_issuer::JwtIssuer, oauth_client::OAuthProviderClient},
        types::session::SessionCookieConfig,
    },
    config::types::Config,
    db::connect::postgres_connect,
    memcached::connect::memcached_connect,
};

pub async fn make_app_state(config: &Config) -> Result<AppState> {
    let postgres_state = Arc::new(postgres_connect(config).await?);
    let oauth_provider_state = Arc::new(OAuthProviderClient::new(config)?);
    let memcached_state = Arc::new(memcached_connect(config)?);
    let jwt_issuer = Arc::new(JwtIssuer::new(config).await?);
    let session_config = Arc::new(SessionCookieConfig::from(&config.security.session));

    Ok(AppState {
        oauth_provider_state,
        postgres_state,
        memcached_state,
        jwt_issuer,
        session_config,
    })
}
