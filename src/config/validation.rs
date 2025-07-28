use crate::config::types::Config;
use anyhow::Result;

// if you need,
// you can add validation logic here
pub(super) fn check_config_validation(config: Config) -> Result<Config> {
    // logic
    Ok(config)
}
