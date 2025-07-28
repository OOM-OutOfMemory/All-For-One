use std::{fs::read_to_string, path::Path};

use anyhow::{Context, Result};

use crate::config::read::read_config;

mod api;
mod config;
mod memcached;
mod provider;
mod user;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = std::env::var("CONFIG").unwrap_or("config.toml".to_string());
    let path = Path::new(&config_path);
    let config = read_config(path)?;
    println!("{:#?}", config);
    Ok(())
}
