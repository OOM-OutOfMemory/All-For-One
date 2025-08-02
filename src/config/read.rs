use anyhow::{Context, Result};
use std::{fs::read_to_string, path::Path};

use crate::config::types::Config;

pub fn read_config() -> Result<Config> {
    let config_path_str = std::env::var("CONFIG").unwrap_or("config.toml".to_string());
    let path = Path::new(&config_path_str);
    let config = toml::from_str::<Config>(
        read_to_string(path)
            .context("Fail to read config file.")?
            .as_str(),
    )
    .context("Fail to parse config file")?;
    Ok(config)
}
