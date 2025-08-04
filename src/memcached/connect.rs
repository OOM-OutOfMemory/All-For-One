use anyhow::{Context, Result};
use deadpool::managed::Pool;
use deadpool_memcached::Manager;

use crate::config::types::Config;

pub fn memcached_connect(config: &Config) -> Result<Pool<Manager>> {
    let addr = format!(
        "{}:{}",
        config.memcached.connect_info.address, config.memcached.connect_info.port
    );
    let manager = Manager::new(addr);
    let memecached_config =
        deadpool_memcached::PoolConfig::new(config.memcached.runtime_options.pool_size as usize);

    let pool: Pool<Manager> = Pool::builder(manager)
        .config(memecached_config)
        .build()
        .context("fail to make memecached connection pool")?;

    Ok(pool)
}
