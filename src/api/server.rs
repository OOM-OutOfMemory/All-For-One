use anyhow::{Context, Result};
use tracing::info;

use crate::{
    api::{router::make_server_route, state::init::make_app_state},
    config::types::Config,
};

pub async fn server_start(config: Config) -> Result<()> {
    let app_state = make_app_state(&config).await?;
    let service = make_server_route(app_state).await;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.server.port))
        .await
        .context("fail to make binding server address")?;

    info!("http server is running");
    Ok(axum::serve(listener, service)
        .await
        .context("fail to start server")?)
}
