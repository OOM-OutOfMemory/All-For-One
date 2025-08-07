use sonic_rs::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: Server,
    pub logger: LoggerConfig,
    pub postgres: PostgresConfig,
    pub memcached: MemCachedConfig,
    pub jwks: JwksConfig,
    pub oidc: OIDCProviderConfig,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub domain: String,
    pub port: u16,
    pub user_agent: String,
}

#[derive(Deserialize, Debug)]
pub struct LoggerConfig {
    pub level: String,
}

#[derive(Deserialize, Debug)]
pub struct PostgresConfig {
    pub connect_info: PostgresConnectConfig,
    pub runtime_options: PostgresRuntimeConfig,
}

#[derive(Deserialize, Debug)]
pub struct PostgresConnectConfig {
    pub address: String,
    pub port: u32,
    pub username: String,
    pub password: String,
    pub db_name: String,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct MemCachedConfig {
    pub connect_info: MemeCachedConnectConfig,
    pub runtime_options: MemCachedRuntimeConfig,
}

#[derive(Deserialize, Debug)]
pub struct MemeCachedConnectConfig {
    pub address: String,
    pub port: u32,
}

#[derive(Deserialize, Debug)]
pub struct MemCachedRuntimeConfig {
    pub init_flush: bool,
    pub pool_size: u32,
    pub read_timeout: u64,
    pub write_timeout: u64,
}

#[derive(Deserialize, Debug)]
pub struct JwksConfig {
    pub iss: String,
    pub aud: String,
    pub keys_path: String,
    pub keys: Vec<KeyConfig>,
}

#[derive(Deserialize, Debug)]
pub struct KeyConfig {
    pub kid: Uuid,
}

#[derive(Deserialize, Debug)]
pub struct OIDCProviderConfig {
    pub github: GithubConfig,
}

#[derive(Deserialize, Debug)]
pub struct GithubConfig {
    pub client_id: String,
    pub client_secret: String,
    pub resource_url: String,
    pub auth_url: String,
    pub token_url: String,
}
