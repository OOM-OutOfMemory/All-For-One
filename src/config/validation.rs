use anyhow::{Result, anyhow};
use std::path::Path;
use url::Url;

use super::types::Config;

pub fn check_config_validation(config: Config) -> Result<Config> {
    validate_server(&config)?;
    validate_logger(&config)?;
    validate_postgres(&config)?;
    validate_memcached(&config)?;
    validate_jwks(&config)?;
    validate_github_config(&config)?;
    validate_security(&config)?;

    Ok(config)
}

fn validate_server(config: &Config) -> Result<()> {
    // Validate domain is a valid URL
    Url::parse(&config.server.domain)
        .map_err(|_| anyhow!("Invalid server domain URL: {}", config.server.domain))?;

    // Validate port range
    if config.server.port == 0 {
        return Err(anyhow!("Server port cannot be 0"));
    }

    // Validate user agent is not empty
    if config.server.user_agent.trim().is_empty() {
        return Err(anyhow!("Server user_agent cannot be empty"));
    }

    Ok(())
}

fn validate_logger(config: &Config) -> Result<()> {
    let valid_levels = ["trace", "debug", "info", "warn", "error"];
    if !valid_levels.contains(&config.logger.level.to_lowercase().as_str()) {
        return Err(anyhow!(
            "Invalid logger level: {}. Must be one of: {}",
            config.logger.level,
            valid_levels.join(", ")
        ));
    }

    Ok(())
}

fn validate_postgres(config: &Config) -> Result<()> {
    let pg = &config.postgres;

    // Validate connection info
    if pg.connect_info.address.trim().is_empty() {
        return Err(anyhow!("Postgres address cannot be empty"));
    }

    if pg.connect_info.port == 0 || pg.connect_info.port > 65535 {
        return Err(anyhow!("Invalid postgres port: {}", pg.connect_info.port));
    }

    if pg.connect_info.username.trim().is_empty() {
        return Err(anyhow!("Postgres username cannot be empty"));
    }

    if pg.connect_info.password.trim().is_empty() {
        return Err(anyhow!("Postgres password cannot be empty"));
    }

    if pg.connect_info.db_name.trim().is_empty() {
        return Err(anyhow!("Postgres database name cannot be empty"));
    }

    // Validate runtime options
    if pg.runtime_options.max_pool_size == 0 {
        return Err(anyhow!("Postgres max_pool_size must be greater than 0"));
    }

    if pg.runtime_options.min_pool_size > pg.runtime_options.max_pool_size {
        return Err(anyhow!(
            "Postgres min_pool_size ({}) cannot be greater than max_pool_size ({})",
            pg.runtime_options.min_pool_size,
            pg.runtime_options.max_pool_size
        ));
    }

    // Validate log level
    let valid_levels = ["trace", "debug", "info", "warn", "error"];
    if !valid_levels.contains(&pg.runtime_options.log_level.to_lowercase().as_str()) {
        return Err(anyhow!(
            "Invalid postgres log level: {}. Must be one of: {}",
            pg.runtime_options.log_level,
            valid_levels.join(", ")
        ));
    }

    Ok(())
}

fn validate_memcached(config: &Config) -> Result<()> {
    let mc = &config.memcached;

    // Validate connection info
    if mc.connect_info.address.trim().is_empty() {
        return Err(anyhow!("Memcached address cannot be empty"));
    }

    if mc.connect_info.port == 0 || mc.connect_info.port > 65535 {
        return Err(anyhow!("Invalid memcached port: {}", mc.connect_info.port));
    }

    // Validate runtime options
    if mc.runtime_options.pool_size == 0 {
        return Err(anyhow!("Memcached pool_size must be greater than 0"));
    }

    Ok(())
}

fn validate_jwks(config: &Config) -> Result<()> {
    let jwks = &config.jwks;

    // Validate issuer URL
    Url::parse(&jwks.iss).map_err(|_| anyhow!("Invalid JWKS issuer URL: {}", jwks.iss))?;

    // Validate audience is not empty
    if jwks.aud.trim().is_empty() {
        return Err(anyhow!("JWKS audience cannot be empty"));
    }

    // Validate keys path exists
    let keys_path = Path::new(&jwks.keys_path);
    if !keys_path.exists() {
        return Err(anyhow!("JWKS keys path does not exist: {}", jwks.keys_path));
    }

    if !keys_path.is_dir() {
        return Err(anyhow!(
            "JWKS keys path is not a directory: {}",
            jwks.keys_path
        ));
    }

    // Validate that keys exist
    if jwks.keys.is_empty() {
        return Err(anyhow!("At least one JWKS key must be configured"));
    }

    Ok(())
}

fn validate_github_config(config: &Config) -> Result<()> {
    let github = &config.oidc.github;

    // Validate GitHub OIDC configuration
    if github.client_id.trim().is_empty() {
        return Err(anyhow!("GitHub client_id cannot be empty"));
    }

    if github.client_secret.trim().is_empty() {
        return Err(anyhow!("GitHub client_secret cannot be empty"));
    }

    // Validate URLs
    Url::parse(&github.resource_url)
        .map_err(|_| anyhow!("Invalid GitHub resource URL: {}", github.resource_url))?;

    Url::parse(&github.auth_url)
        .map_err(|_| anyhow!("Invalid GitHub auth URL: {}", github.auth_url))?;

    Url::parse(&github.token_url)
        .map_err(|_| anyhow!("Invalid GitHub token URL: {}", github.token_url))?;

    Ok(())
}

fn validate_security(config: &Config) -> Result<()> {
    let security = &config.security;

    validate_jwt_security(&security.jwt)?;
    validate_session_security(&security.session)?;
    validate_rate_limiting(&security.rate_limiting)?;
    validate_cors(&security.cors)?;
    validate_security_headers(&security.security_headers)?;

    Ok(())
}

fn validate_jwt_security(jwt: &super::types::JwtSecurityConfig) -> Result<()> {
    // Validate TTL values are reasonable
    if jwt.access_token_ttl == 0 {
        return Err(anyhow!("JWT access_token_ttl must be greater than 0"));
    }

    if jwt.refresh_token_ttl == 0 {
        return Err(anyhow!("JWT refresh_token_ttl must be greater than 0"));
    }

    if jwt.access_token_ttl >= jwt.refresh_token_ttl {
        return Err(anyhow!(
            "JWT access_token_ttl ({}) should be less than refresh_token_ttl ({})",
            jwt.access_token_ttl,
            jwt.refresh_token_ttl
        ));
    }

    if jwt.key_rotation_interval == 0 {
        return Err(anyhow!("JWT key_rotation_interval must be greater than 0"));
    }

    // Validate algorithm
    // only EdDSA is supported for now
    let valid_algorithms = ["EdDSA"];
    if !valid_algorithms.contains(&jwt.algorithm.as_str()) {
        return Err(anyhow!(
            "Invalid JWT algorithm: {}. Must be one of: {}",
            jwt.algorithm,
            valid_algorithms.join(", ")
        ));
    }

    Ok(())
}

fn validate_session_security(session: &super::types::SessionSecurityConfig) -> Result<()> {
    if session.cookie_ttl == 0 {
        return Err(anyhow!("Session cookie_ttl must be greater than 0"));
    }

    if session.cache_ttl == 0 {
        return Err(anyhow!("Session cache_ttl must be greater than 0"));
    }

    // Validate same_site values
    let valid_same_site = ["Strict", "Lax", "None"];
    if !valid_same_site.contains(&session.same_site.as_str()) {
        return Err(anyhow!(
            "Invalid session same_site value: {}. Must be one of: {}",
            session.same_site,
            valid_same_site.join(", ")
        ));
    }

    Ok(())
}

fn validate_rate_limiting(rate_limit: &super::types::RateLimitingConfig) -> Result<()> {
    if rate_limit.enabled {
        if rate_limit.requests_per_minute == 0 {
            return Err(anyhow!(
                "Rate limiting requests_per_minute must be greater than 0 when enabled"
            ));
        }

        if rate_limit.burst_size == 0 {
            return Err(anyhow!(
                "Rate limiting burst_size must be greater than 0 when enabled"
            ));
        }

        if rate_limit.cleanup_interval == 0 {
            return Err(anyhow!(
                "Rate limiting cleanup_interval must be greater than 0 when enabled"
            ));
        }
    }

    Ok(())
}

fn validate_cors(cors: &super::types::CorsConfig) -> Result<()> {
    if cors.enabled {
        if cors.allowed_origins.is_empty() {
            return Err(anyhow!(
                "CORS allowed_origins cannot be empty when CORS is enabled"
            ));
        }

        if cors.allowed_methods.is_empty() {
            return Err(anyhow!(
                "CORS allowed_methods cannot be empty when CORS is enabled"
            ));
        }

        if cors.allowed_headers.is_empty() {
            return Err(anyhow!(
                "CORS allowed_headers cannot be empty when CORS is enabled"
            ));
        }

        // Validate HTTP methods
        let valid_methods = ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"];
        for method in &cors.allowed_methods {
            if !valid_methods.contains(&method.to_uppercase().as_str()) {
                return Err(anyhow!(
                    "Invalid CORS method: {}. Must be one of: {}",
                    method,
                    valid_methods.join(", ")
                ));
            }
        }
    }

    Ok(())
}

fn validate_security_headers(headers: &super::types::SecurityHeadersConfig) -> Result<()> {
    if headers.hsts_enabled && headers.hsts_max_age == 0 {
        return Err(anyhow!(
            "HSTS max_age must be greater than 0 when HSTS is enabled"
        ));
    }

    if headers.csp_enabled && headers.csp_policy.trim().is_empty() {
        return Err(anyhow!("CSP policy cannot be empty when CSP is enabled"));
    }

    // Validate X-Frame-Options values
    let valid_frame_options = ["DENY", "SAMEORIGIN"];
    if !valid_frame_options.contains(&headers.x_frame_options.as_str()) {
        return Err(anyhow!(
            "Invalid X-Frame-Options value: {}. Must be one of: {}",
            headers.x_frame_options,
            valid_frame_options.join(", ")
        ));
    }

    Ok(())
}
