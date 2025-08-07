#[cfg(test)]
mod tests {
    use crate::memcached::types::AuthVerifyToken;

    #[test]
    fn test_auth_verify_token_serialization() {
        let token = AuthVerifyToken {
            csrf_token: "test-csrf-token".to_string(),
            pkce_verifier: "test-pkce-verifier".to_string(),
            nonce: Some("test-nonce".to_string()),
        };

        let serialized = sonic_rs::to_string(&token).unwrap();
        assert!(serialized.contains("test-csrf-token"));
        assert!(serialized.contains("test-pkce-verifier"));
        assert!(serialized.contains("test-nonce"));
    }

    #[test]
    fn test_auth_verify_token_deserialization() {
        let json = r#"{
            "csrf_token": "test-csrf-token",
            "pkce_verifier": "test-pkce-verifier",
            "nonce": "test-nonce"
        }"#;

        let token: AuthVerifyToken = sonic_rs::from_str(json).unwrap();
        assert_eq!(token.csrf_token, "test-csrf-token");
        assert_eq!(token.pkce_verifier, "test-pkce-verifier");
        assert_eq!(token.nonce, Some("test-nonce".to_string()));
    }

    #[test]
    fn test_auth_verify_token_without_nonce() {
        let json = r#"{
            "csrf_token": "test-csrf-token",
            "pkce_verifier": "test-pkce-verifier",
            "nonce": null
        }"#;

        let token: AuthVerifyToken = sonic_rs::from_str(json).unwrap();
        assert_eq!(token.csrf_token, "test-csrf-token");
        assert_eq!(token.pkce_verifier, "test-pkce-verifier");
        assert_eq!(token.nonce, None);
    }

    #[test]
    fn test_auth_verify_token_debug() {
        let token = AuthVerifyToken {
            csrf_token: "test-csrf-token".to_string(),
            pkce_verifier: "test-pkce-verifier".to_string(),
            nonce: Some("test-nonce".to_string()),
        };

        let debug_str = format!("{:?}", token);
        assert!(debug_str.contains("test-csrf-token"));
        assert!(debug_str.contains("test-pkce-verifier"));
        assert!(debug_str.contains("test-nonce"));
    }

    #[test]
    fn test_auth_verify_token_missing_fields() {
        let json = r#"{
            "csrf_token": "test-csrf-token"
        }"#;

        let result: Result<AuthVerifyToken, _> = sonic_rs::from_str(json);
        assert!(result.is_err());
    }
}
