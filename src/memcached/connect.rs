use anyhow::{Context, Result};
use memcache::Client;
use url::Url;

pub fn memcached_connect(url: Url, pool_size: u32) -> Result<Client> {
    Ok(memcache::Client::with_pool_size(url, pool_size)
        .context("Fail to connect memcached server")?)
}
