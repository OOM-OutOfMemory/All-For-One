use crate::config::types::JwtSecurityConfig;

/// JWT 보안 설정을 미리 변환해서 저장하는 구조체
#[derive(Clone)]
pub struct JwtConfig {
    pub access_token_ttl: i64,
    pub refresh_token_ttl: i64,
    pub key_rotation_interval: u64,
    pub algorithm: String,
}

impl From<&JwtSecurityConfig> for JwtConfig {
    fn from(config: &JwtSecurityConfig) -> Self {
        Self {
            access_token_ttl: config.access_token_ttl as i64,
            refresh_token_ttl: config.refresh_token_ttl as i64,
            key_rotation_interval: config.key_rotation_interval,
            algorithm: config.algorithm.clone(),
        }
    }
}