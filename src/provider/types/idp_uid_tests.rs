#[cfg(test)]
mod tests {
    use crate::provider::types::idp_uid::GithubUid;

    #[test]
    fn test_github_uid_deserialization() {
        let json = r#"{"id": 12345}"#;
        let uid: GithubUid = sonic_rs::from_str(json).unwrap();
        assert_eq!(uid.id, 12345);
    }

    #[test]
    fn test_github_uid_negative_id() {
        let json = r#"{"id": -1}"#;
        let uid: GithubUid = sonic_rs::from_str(json).unwrap();
        assert_eq!(uid.id, -1);
    }

    #[test]
    fn test_github_uid_zero_id() {
        let json = r#"{"id": 0}"#;
        let uid: GithubUid = sonic_rs::from_str(json).unwrap();
        assert_eq!(uid.id, 0);
    }

    #[test]
    fn test_github_uid_large_id() {
        let json = r#"{"id": 9223372036854775807}"#; // i64::MAX
        let uid: GithubUid = sonic_rs::from_str(json).unwrap();
        assert_eq!(uid.id, 9223372036854775807);
    }

    #[test]
    fn test_github_uid_invalid_json() {
        let json = r#"{"invalid": "field"}"#;
        let result: Result<GithubUid, _> = sonic_rs::from_str(json);
        assert!(result.is_err());
    }
}
