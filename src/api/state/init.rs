use std::sync::Arc;

use anyhow::{Context, Result};

use crate::{
    api::state::types::{app::AppState, oauth_client::OAuthProviderClient},
    config::types::Config,
    db::connect::postgres_connect,
    memcached::connect::memcached_connect,
};

pub async fn make_app_state(config: &Config) -> Result<AppState> {
    let postgres_state = Arc::new(postgres_connect(config).await?);
    let oauth_provider_state = Arc::new(OAuthProviderClient::new(config)?);
    let memcached_state = Arc::new(memcached_connect(config)?);

    Ok(AppState {
        oauth_provider_state,
        postgres_state,
        memcached_state,
    })
}
