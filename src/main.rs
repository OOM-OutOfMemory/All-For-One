use anyhow::Result;
use tracing::info;

use crate::{
    config::read::read_config,
    db::connect::postgres_connect,
    memcached::connect::memcached_connect,
    utils::{logger::init_logger, types::HTTP_REQUEST_USER_AGENT},
};

mod api;
mod config;
mod db;
mod entity;
mod memcached;
mod provider;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let config = read_config()?;

    HTTP_REQUEST_USER_AGENT.get_or_init(|| config.server.user_agent.clone());

    init_logger(&config)?;
    info!("logger initialized");

    memcached_connect(&config)?;
    info!("memcached connected");

    postgres_connect(&config).await?;
    info!("postgres connected");

    info!("server will be started");
    api::server::server_start(config).await?;
    Ok(())
}
