use anyhow::{Result, anyhow};
use tracing::Level;

use crate::config::types::Config;

pub fn init_logger(config: &Config) -> Result<()> {
    let level = match config.logger.level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => {
            return Err(anyhow!(
                "Unsupported log level: {}",
                config.logger.level.to_lowercase().as_str()
            ));
        }
    };
    tracing_subscriber::fmt().with_max_level(level).init();
    Ok(())
}
