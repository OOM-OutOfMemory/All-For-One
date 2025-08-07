#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use crate::api::types::jwt_claim::Claims;

    #[test]
    fn test_claims_creation() {
        let user_id = Uuid::now_v7();
        let jwt_id = Uuid::now_v7();
        let now = Utc::now().timestamp();

        let claims = Claims {
            aud: "test-audience".to_string(),
            iss: "test-issuer".to_string(),
            sub: user_id,
            exp: now + 3600,
            jti: jwt_id,
            iat: now,
            nbf: now,
        };

        assert_eq!(claims.aud, "test-audience");
        assert_eq!(claims.iss, "test-issuer");
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.exp, now + 3600);
        assert_eq!(claims.jti, jwt_id);
        assert_eq!(claims.iat, now);
        assert_eq!(claims.nbf, now);
    }

    #[test]
    fn test_claims_serialization() {
        let user_id = Uuid::now_v7();
        let jwt_id = Uuid::now_v7();
        let now = Utc::now().timestamp();

        let claims = Claims {
            aud: "test-audience".to_string(),
            iss: "test-issuer".to_string(),
            sub: user_id,
            exp: now + 3600,
            jti: jwt_id,
            iat: now,
            nbf: now,
        };

        let serialized = sonic_rs::to_string(&claims).unwrap();
        assert!(serialized.contains("test-audience"));
        assert!(serialized.contains("test-issuer"));
        assert!(serialized.contains(&user_id.to_string()));
    }

    #[test]
    fn test_claims_deserialization() {
        let user_id = Uuid::now_v7();
        let jwt_id = Uuid::now_v7();
        let now = Utc::now().timestamp();

        let json = format!(
            r#"{{
                "aud": "test-audience",
                "iss": "test-issuer",
                "sub": "{}",
                "exp": {},
                "jti": "{}",
                "iat": {},
                "nbf": {}
            }}"#,
            user_id,
            now + 3600,
            jwt_id,
            now,
            now
        );

        let claims: Claims = sonic_rs::from_str(&json).unwrap();
        assert_eq!(claims.aud, "test-audience");
        assert_eq!(claims.iss, "test-issuer");
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.exp, now + 3600);
        assert_eq!(claims.jti, jwt_id);
        assert_eq!(claims.iat, now);
        assert_eq!(claims.nbf, now);
    }

    #[test]
    fn test_claims_debug() {
        let user_id = Uuid::now_v7();
        let jwt_id = Uuid::now_v7();
        let now = Utc::now().timestamp();

        let claims = Claims {
            aud: "test-audience".to_string(),
            iss: "test-issuer".to_string(),
            sub: user_id,
            exp: now + 3600,
            jti: jwt_id,
            iat: now,
            nbf: now,
        };

        let debug_str = format!("{:?}", claims);
        assert!(debug_str.contains("test-audience"));
        assert!(debug_str.contains("test-issuer"));
    }
}
