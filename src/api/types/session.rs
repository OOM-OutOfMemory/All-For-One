use axum_extra::extract::cookie::{Cookie, SameSite};
use crate::{api::types::cookie::COOKIE_AUTH_REQUEST_ID, config::types::SessionSecurityConfig};

/// 세션 쿠키 설정을 미리 변환해서 저장하는 구조체
#[derive(Clone)]
pub struct SessionCookieConfig {
    pub cookie_ttl: i64,
    pub cache_ttl: u64,
    pub secure_cookies: bool,
    pub same_site: SameSite,
    pub http_only: bool,
}

impl From<&SessionSecurityConfig> for SessionCookieConfig {
    fn from(config: &SessionSecurityConfig) -> Self {
        let same_site = match config.same_site.as_str() {
            "Strict" => SameSite::Strict,
            "Lax" => SameSite::Lax,
            "None" => SameSite::None,
            _ => SameSite::Lax, // 기본값
        };

        Self {
            cookie_ttl: config.cookie_ttl as i64,
            cache_ttl: config.cache_ttl,
            secure_cookies: config.secure_cookies,
            same_site,
            http_only: config.http_only,
        }
    }
}

impl SessionCookieConfig {
    /// 세션 쿠키 생성
    pub fn create_session_cookie(&self, session_id: &str) -> Cookie<'static> {
        Cookie::build((COOKIE_AUTH_REQUEST_ID, session_id.to_string()))
            .http_only(self.http_only)
            .secure(self.secure_cookies)
            .same_site(self.same_site)
            .max_age(cookie::time::Duration::seconds(self.cookie_ttl))
            .path("/")
            .build()
    }

    /// 세션 쿠키 삭제
    pub fn create_session_removal_cookie(&self) -> Cookie<'static> {
        let mut cookie = Cookie::build((COOKIE_AUTH_REQUEST_ID, ""))
            .http_only(self.http_only)
            .secure(self.secure_cookies)
            .same_site(self.same_site)
            .path("/")
            .build();
        
        cookie.make_removal();
        cookie
    }
}