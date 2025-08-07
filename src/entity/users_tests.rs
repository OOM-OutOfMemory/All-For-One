#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use uuid::Uuid;

    use crate::entity::users::Model;

    #[test]
    fn test_user_model_creation() {
        let user_id = Uuid::now_v7();
        let now: DateTime<Utc> = Utc::now();
        let user = Model {
            id: user_id,
            username: Some("testuser".to_string()),
            email: Some("test@example.com".to_string()),
            is_active: true,
            created_at: now.into(),
            updated_at: now.into(),
            idp: "github".to_string(),
            idp_uid: "12345".to_string(),
        };

        assert_eq!(user.id, user_id);
        assert_eq!(user.username, Some("testuser".to_string()));
        assert_eq!(user.email, Some("test@example.com".to_string()));
        assert!(user.is_active);
        assert_eq!(user.idp, "github");
        assert_eq!(user.idp_uid, "12345");
    }

    #[test]
    fn test_user_model_optional_fields() {
        let user_id = Uuid::now_v7();
        let now: DateTime<Utc> = Utc::now();
        let user = Model {
            id: user_id,
            username: None,
            email: None,
            is_active: false,
            created_at: now.into(),
            updated_at: now.into(),
            idp: "github".to_string(),
            idp_uid: "67890".to_string(),
        };

        assert_eq!(user.username, None);
        assert_eq!(user.email, None);
        assert!(!user.is_active);
    }

    #[test]
    fn test_user_model_serialization() {
        let user_id = Uuid::now_v7();
        let now: DateTime<Utc> = Utc::now();
        let user = Model {
            id: user_id,
            username: Some("testuser".to_string()),
            email: Some("test@example.com".to_string()),
            is_active: true,
            created_at: now.into(),
            updated_at: now.into(),
            idp: "github".to_string(),
            idp_uid: "12345".to_string(),
        };

        let serialized = sonic_rs::to_string(&user).unwrap();
        assert!(serialized.contains("testuser"));
        assert!(serialized.contains("test@example.com"));
        assert!(serialized.contains("github"));
        assert!(serialized.contains("12345"));
    }

    #[test]
    fn test_user_model_deserialization() {
        let user_id = Uuid::now_v7();
        let now = Utc::now();

        let json = format!(
            r#"{{
                "id": "{}",
                "username": "testuser",
                "email": "test@example.com",
                "is_active": true,
                "created_at": "{}",
                "updated_at": "{}",
                "idp": "github",
                "idp_uid": "12345"
            }}"#,
            user_id,
            now.to_rfc3339(),
            now.to_rfc3339()
        );

        let user: Model = sonic_rs::from_str(&json).unwrap();
        assert_eq!(user.id, user_id);
        assert_eq!(user.username, Some("testuser".to_string()));
        assert_eq!(user.email, Some("test@example.com".to_string()));
        assert!(user.is_active);
        assert_eq!(user.idp, "github");
        assert_eq!(user.idp_uid, "12345");
    }

    #[test]
    fn test_user_model_equality() {
        let user_id = Uuid::now_v7();
        let now: DateTime<Utc> = Utc::now();

        let user1 = Model {
            id: user_id,
            username: Some("testuser".to_string()),
            email: Some("test@example.com".to_string()),
            is_active: true,
            created_at: now.into(),
            updated_at: now.into(),
            idp: "github".to_string(),
            idp_uid: "12345".to_string(),
        };

        let user2 = Model {
            id: user_id,
            username: Some("testuser".to_string()),
            email: Some("test@example.com".to_string()),
            is_active: true,
            created_at: now.into(),
            updated_at: now.into(),
            idp: "github".to_string(),
            idp_uid: "12345".to_string(),
        };

        assert_eq!(user1, user2);
    }

    #[test]
    fn test_user_model_debug() {
        let user_id = Uuid::now_v7();
        let now: DateTime<Utc> = Utc::now();

        let user = Model {
            id: user_id,
            username: Some("testuser".to_string()),
            email: Some("test@example.com".to_string()),
            is_active: true,
            created_at: now.into(),
            updated_at: now.into(),
            idp: "github".to_string(),
            idp_uid: "12345".to_string(),
        };

        let debug_str = format!("{:?}", user);
        assert!(debug_str.contains("testuser"));
        assert!(debug_str.contains("github"));
    }
}
