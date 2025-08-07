#[cfg(test)]
mod tests {
    use crate::api::types::{cookie::COOKIE_AUTH_REQUEST_ID, session::SessionCookieConfig};
    use axum_extra::extract::cookie::SameSite;

    #[test]
    fn test_session_cookie_config_from_security_config() {
        let security_config = crate::config::types::SessionSecurityConfig {
            cookie_ttl: 300,
            cache_ttl: 600,
            secure_cookies: true,
            same_site: "Lax".to_string(),
            http_only: true,
        };

        let session_config = SessionCookieConfig::from(&security_config);

        assert_eq!(session_config.cookie_ttl, 300);
        assert_eq!(session_config.cache_ttl, 600);
        assert!(session_config.secure_cookies);
        assert_eq!(session_config.same_site, SameSite::Lax);
        assert!(session_config.http_only);
    }

    #[test]
    fn test_session_cookie_config_same_site_strict() {
        let security_config = crate::config::types::SessionSecurityConfig {
            cookie_ttl: 300,
            cache_ttl: 600,
            secure_cookies: true,
            same_site: "Strict".to_string(),
            http_only: true,
        };

        let session_config = SessionCookieConfig::from(&security_config);
        assert_eq!(session_config.same_site, SameSite::Strict);
    }

    #[test]
    fn test_session_cookie_config_same_site_none() {
        let security_config = crate::config::types::SessionSecurityConfig {
            cookie_ttl: 300,
            cache_ttl: 600,
            secure_cookies: true,
            same_site: "None".to_string(),
            http_only: true,
        };

        let session_config = SessionCookieConfig::from(&security_config);
        assert_eq!(session_config.same_site, SameSite::None);
    }

    #[test]
    fn test_session_cookie_config_same_site_invalid_defaults_to_lax() {
        let security_config = crate::config::types::SessionSecurityConfig {
            cookie_ttl: 300,
            cache_ttl: 600,
            secure_cookies: true,
            same_site: "Invalid".to_string(),
            http_only: true,
        };

        let session_config = SessionCookieConfig::from(&security_config);
        assert_eq!(session_config.same_site, SameSite::Lax);
    }

    #[test]
    fn test_create_session_cookie() {
        let session_config = SessionCookieConfig {
            cookie_ttl: 300,
            cache_ttl: 600,
            secure_cookies: true,
            same_site: SameSite::Lax,
            http_only: true,
        };

        let session_id = "test-session-id";
        let cookie = session_config.create_session_cookie(session_id);

        assert_eq!(cookie.name(), COOKIE_AUTH_REQUEST_ID);
        assert_eq!(cookie.value(), session_id);
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.secure(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
        assert_eq!(cookie.path(), Some("/"));
    }

    #[test]
    fn test_create_session_removal_cookie() {
        let session_config = SessionCookieConfig {
            cookie_ttl: 300,
            cache_ttl: 600,
            secure_cookies: true,
            same_site: SameSite::Lax,
            http_only: true,
        };

        let cookie = session_config.create_session_removal_cookie();

        assert_eq!(cookie.name(), COOKIE_AUTH_REQUEST_ID);
        assert_eq!(cookie.value(), "");
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.secure(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
        assert_eq!(cookie.path(), Some("/"));
    }
}
