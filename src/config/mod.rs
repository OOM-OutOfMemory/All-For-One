use sonic_rs::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    logging: LoggerConfig,
    memcached: MemCachedConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggerConfig {
    level: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemCachedConfig {
    connect_info: MemeCachedConnectConfig,
    runtime_options: MemCachedRuntimeConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemeCachedConnectConfig {
    address: String,
    port: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct MemCachedRuntimeConfig {
    init_flush: bool,
    pool_size: u32,
    read_timeout: u64,
    write_timeout: u64,
}

impl Default for MemCachedRuntimeConfig {
    fn default() -> Self {
        MemCachedRuntimeConfig {
            init_flush: false,
            pool_size: 10,
            read_timeout: 60,
            write_timeout: 60,
        }
    }
}
