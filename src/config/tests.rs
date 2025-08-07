#[cfg(test)]
mod tests {
    use std::io::Write;
    use tempfile::NamedTempFile;

    use crate::config::{read, types::*, validation};

    #[test]
    fn test_valid_config_parsing() {
        let config_content = r#"
[server]
domain = "http://127.0.0.1"
port = 3000
user_agent = "AllForOne/0.1.0"

[logger]
level = "debug"

[memcached]
[memcached.connect_info]
address = "127.0.0.1"
port = 11211
[memcached.runtime_options]
init_flush = false
pool_size = 10
read_timeout = 60
write_timeout = 60

[postgres]
[postgres.connect_info]
address = "127.0.0.1"
port = 5432
username = "test_user"
password = "test_pass"
db_name = "test_db"
[postgres.runtime_options]
max_pool_size = 10
min_pool_size = 5
connect_timeout = 8
acquire_timeout = 8
idle_timeout = 8
max_lifetime = 8
sqlx_logging = true
log_level = "debug"

[jwks]
iss = "https://auth.example.com"
aud = "AllForOne-Project-Service"
keys_path = "./test_jwks"

[[jwks.keys]]
kid = "13f03b9f-f209-4dcd-86f0-69cc19e773eb"

[oidc]
[oidc.github]
client_id = "test_client_id"
client_secret = "test_client_secret"
resource_url = "https://api.github.com"
auth_url = "https://github.com/login/oauth/authorize"
token_url = "https://github.com/login/oauth/access_token"

[security]
[security.jwt]
access_token_ttl = 900
refresh_token_ttl = 86400
key_rotation_interval = 2592000
algorithm = "EdDSA"

[security.session]
cookie_ttl = 300
cache_ttl = 600
secure_cookies = true
same_site = "Lax"
http_only = true

[security.rate_limiting]
enabled = true
requests_per_minute = 60
burst_size = 10
cleanup_interval = 300

[security.cors]
enabled = true
allowed_origins = ["https://example.com"]
allowed_methods = ["GET", "POST"]
allowed_headers = ["Content-Type", "Authorization"]
max_age = 3600

[security.security_headers]
hsts_enabled = true
hsts_max_age = 31536000
csp_enabled = true
csp_policy = "default-src 'self'"
x_frame_options = "DENY"
x_content_type_options = true
"#;

        let config: Config = toml::from_str(config_content).unwrap();

        assert_eq!(config.server.domain, "http://127.0.0.1");
        assert_eq!(config.server.port, 3000);
        assert_eq!(config.logger.level, "debug");
        assert_eq!(config.postgres.connect_info.db_name, "test_db");
        assert_eq!(config.oidc.github.client_id, "test_client_id");
    }

    #[test]
    fn test_config_validation_invalid_domain() {
        let mut config = create_valid_test_config();
        config.server.domain = "invalid-url".to_string();

        let result = validation::check_config_validation(config);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid server domain URL")
        );
    }

    #[test]
    fn test_config_validation_invalid_port() {
        let mut config = create_valid_test_config();
        config.server.port = 0;

        let result = validation::check_config_validation(config);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Server port cannot be 0")
        );
    }

    #[test]
    fn test_config_validation_invalid_log_level() {
        let mut config = create_valid_test_config();
        config.logger.level = "invalid".to_string();

        let result = validation::check_config_validation(config);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid logger level")
        );
    }

    #[test]
    fn test_config_validation_empty_postgres_username() {
        let mut config = create_valid_test_config();
        config.postgres.connect_info.username = "".to_string();

        let result = validation::check_config_validation(config);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Postgres username cannot be empty")
        );
    }

    #[test]
    fn test_config_validation_invalid_jwt_ttl() {
        let mut config = create_valid_test_config();
        config.security.jwt.access_token_ttl = 0;

        let result = validation::check_config_validation(config);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("JWT access_token_ttl must be greater than 0")
        );
    }

    fn create_valid_test_config() -> Config {
        Config {
            server: Server {
                domain: "http://127.0.0.1".to_string(),
                port: 3000,
                user_agent: "AllForOne/0.1.0".to_string(),
            },
            logger: LoggerConfig {
                level: "debug".to_string(),
            },
            postgres: PostgresConfig {
                connect_info: PostgresConnectConfig {
                    address: "127.0.0.1".to_string(),
                    port: 5432,
                    username: "test_user".to_string(),
                    password: "test_pass".to_string(),
                    db_name: "test_db".to_string(),
                },
                runtime_options: PostgresRuntimeConfig {
                    max_pool_size: 10,
                    min_pool_size: 5,
                    connect_timeout: 8,
                    acquire_timeout: 8,
                    idle_timeout: 8,
                    max_lifetime: 8,
                    sqlx_logging: true,
                    log_level: "debug".to_string(),
                },
            },
            memcached: MemCachedConfig {
                connect_info: MemeCachedConnectConfig {
                    address: "127.0.0.1".to_string(),
                    port: 11211,
                },
                runtime_options: MemCachedRuntimeConfig {
                    init_flush: false,
                    pool_size: 10,
                    read_timeout: 60,
                    write_timeout: 60,
                },
            },
            jwks: JwksConfig {
                iss: "https://auth.example.com".to_string(),
                aud: "AllForOne-Project-Service".to_string(),
                keys_path: "./test_jwks".to_string(),
                keys: vec![KeyConfig {
                    kid: uuid::Uuid::parse_str("13f03b9f-f209-4dcd-86f0-69cc19e773eb").unwrap(),
                }],
            },
            oidc: OIDCProviderConfig {
                github: GithubConfig {
                    client_id: "test_client_id".to_string(),
                    client_secret: "test_client_secret".to_string(),
                    resource_url: "https://api.github.com".to_string(),
                    auth_url: "https://github.com/login/oauth/authorize".to_string(),
                    token_url: "https://github.com/login/oauth/access_token".to_string(),
                },
            },
            security: SecurityConfig {
                jwt: JwtSecurityConfig {
                    access_token_ttl: 900,
                    refresh_token_ttl: 86400,
                    key_rotation_interval: 2592000,
                    algorithm: "EdDSA".to_string(),
                },
                session: SessionSecurityConfig {
                    cookie_ttl: 300,
                    cache_ttl: 600,
                    secure_cookies: true,
                    same_site: "Lax".to_string(),
                    http_only: true,
                },
                rate_limiting: RateLimitingConfig {
                    enabled: true,
                    requests_per_minute: 60,
                    burst_size: 10,
                    cleanup_interval: 300,
                },
                cors: CorsConfig {
                    enabled: true,
                    allowed_origins: vec!["https://example.com".to_string()],
                    allowed_methods: vec!["GET".to_string(), "POST".to_string()],
                    allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
                    max_age: 3600,
                },
                security_headers: SecurityHeadersConfig {
                    hsts_enabled: true,
                    hsts_max_age: 31536000,
                    csp_enabled: true,
                    csp_policy: "default-src 'self'".to_string(),
                    x_frame_options: "DENY".to_string(),
                    x_content_type_options: true,
                },
            },
        }
    }

    #[test]
    fn test_config_read_from_file() {
        let config_content = r#"
[server]
domain = "http://127.0.0.1"
port = 3000
user_agent = "AllForOne/0.1.0"

[logger]
level = "info"

[memcached]
[memcached.connect_info]
address = "127.0.0.1"
port = 11211
[memcached.runtime_options]
init_flush = false
pool_size = 10
read_timeout = 60
write_timeout = 60

[postgres]
[postgres.connect_info]
address = "127.0.0.1"
port = 5432
username = "test_user"
password = "test_pass"
db_name = "test_db"
[postgres.runtime_options]
max_pool_size = 10
min_pool_size = 5
connect_timeout = 8
acquire_timeout = 8
idle_timeout = 8
max_lifetime = 8
sqlx_logging = true
log_level = "debug"

[jwks]
iss = "https://auth.example.com"
aud = "AllForOne-Project-Service"
keys_path = "./test_jwks"

[[jwks.keys]]
kid = "13f03b9f-f209-4dcd-86f0-69cc19e773eb"

[oidc]
[oidc.github]
client_id = "test_client_id"
client_secret = "test_client_secret"
resource_url = "https://api.github.com"
auth_url = "https://github.com/login/oauth/authorize"
token_url = "https://github.com/login/oauth/access_token"

[security]
[security.jwt]
access_token_ttl = 900
refresh_token_ttl = 86400
key_rotation_interval = 2592000
algorithm = "EdDSA"

[security.session]
cookie_ttl = 300
cache_ttl = 600
secure_cookies = true
same_site = "Lax"
http_only = true

[security.rate_limiting]
enabled = true
requests_per_minute = 60
burst_size = 10
cleanup_interval = 300

[security.cors]
enabled = true
allowed_origins = ["https://example.com"]
allowed_methods = ["GET", "POST"]
allowed_headers = ["Content-Type", "Authorization"]
max_age = 3600

[security.security_headers]
hsts_enabled = true
hsts_max_age = 31536000
csp_enabled = true
csp_policy = "default-src 'self'"
x_frame_options = "DENY"
x_content_type_options = true
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(config_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        unsafe {
            // Set the environment variable to the temp file path
            std::env::set_var("CONFIG", temp_file.path());
        }

        let result = read::read_config();
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.server.domain, "http://127.0.0.1");
        assert_eq!(config.logger.level, "info");
    }
}
