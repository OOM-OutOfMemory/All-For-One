#[cfg(test)]
mod tests {
    use crate::provider::types::idp::OAuthProvider;

    #[test]
    fn test_oauth_provider_as_str() {
        assert_eq!(OAuthProvider::Github.as_str(), "github");
    }

    #[test]
    fn test_oauth_provider_serialization() {
        let provider = OAuthProvider::Github;
        let serialized = sonic_rs::to_string(&provider).unwrap();
        assert_eq!(serialized, "\"github\"");
    }

    #[test]
    fn test_oauth_provider_deserialization() {
        let json = "\"github\"";
        let provider: OAuthProvider = sonic_rs::from_str(json).unwrap();
        match provider {
            OAuthProvider::Github => {
                // Success
            }
        }
    }

    #[test]
    fn test_oauth_provider_debug() {
        let provider = OAuthProvider::Github;
        let debug_str = format!("{:?}", provider);
        assert!(debug_str.contains("Github"));
    }

    #[test]
    fn test_oauth_provider_clone() {
        let provider = OAuthProvider::Github;
        let cloned = provider.clone();
        assert_eq!(provider.as_str(), cloned.as_str());
    }
}
