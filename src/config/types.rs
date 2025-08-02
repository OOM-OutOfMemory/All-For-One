use sonic_rs::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server: Server,
    pub logger: LoggerConfig,
    pub postgres: PostgresConfig,
    pub memcached: MemCachedConfig,
    pub oidc: OIDCProviderConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub domain: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggerConfig {
    pub level: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostgresConfig {
    pub connect_info: PostgresConnectConfig,
    pub runtime_options: PostgresRuntimeConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostgresConnectConfig {
    pub address: String,
    pub port: u32,
    pub username: String,
    pub password: String,
    pub db_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostgresRuntimeConfig {
    pub max_pool_size: u32,
    pub min_pool_size: u32,
    pub connect_timeout: u64,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
    pub sqlx_logging: bool,
    pub log_level: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemCachedConfig {
    pub connect_info: MemeCachedConnectConfig,
    pub runtime_options: MemCachedRuntimeConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemeCachedConnectConfig {
    pub address: String,
    pub port: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct MemCachedRuntimeConfig {
    pub init_flush: bool,
    pub pool_size: u32,
    pub read_timeout: u64,
    pub write_timeout: u64,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct OIDCProviderConfig {
    pub github: GithubConfig,
    // pub google: GoogleConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubConfig {
    pub client_id: String,
    pub client_secret: String,
    pub resource_url: String,
    pub auth_url: String,
    pub token_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoogleConfig {
    pub client_id: String,
    pub client_secret: String,
    pub google_user_api_url: String,
    pub auth_url: String,
    pub token_url: String,
}
