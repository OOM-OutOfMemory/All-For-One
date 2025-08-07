use crate::config::types::Config;
use anyhow::Result;

// if you need,
// you can add validation logic here
pub fn check_config_validation(config: Config) -> Result<Config> {
    eprintln!("[Before Log Init] implement config validation logic");
    Ok(config)
}
