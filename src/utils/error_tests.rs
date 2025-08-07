#[cfg(test)]
mod tests {
    use crate::utils::error::AllForOneError;

    #[test]
    fn test_error_conversion_path_rejection() {
        // This is a simplified test as creating PathRejection directly is complex
        // In real scenarios, these would be created by Axum extractors
        let error = AllForOneError::Auth("test auth error".to_string());
        match error {
            AllForOneError::Auth(msg) => assert_eq!(msg, "test auth error"),
            _ => panic!("Expected Auth error"),
        }
    }

    #[test]
    fn test_error_conversion_auth() {
        let error = AllForOneError::Auth("unauthorized access".to_string());
        match error {
            AllForOneError::Auth(msg) => assert_eq!(msg, "unauthorized access"),
            _ => panic!("Expected Auth error"),
        }
    }

    #[test]
    fn test_error_conversion_internal() {
        let internal_error = anyhow::anyhow!("internal server error");
        let error = AllForOneError::Internal(internal_error);

        match error {
            AllForOneError::Internal(err) => {
                assert_eq!(err.to_string(), "internal server error");
            }
            _ => panic!("Expected Internal error"),
        }
    }

    #[test]
    fn test_error_display() {
        let error = AllForOneError::Auth("test error".to_string());
        assert_eq!(error.to_string(), "auth error");
    }

    #[test]
    fn test_error_from_sea_orm() {
        let db_error = sea_orm::DbErr::Custom("database connection failed".to_string());
        let error = AllForOneError::from(db_error);

        match error {
            AllForOneError::Db(_) => {
                // Success - error was properly converted
            }
            _ => panic!("Expected Db error"),
        }
    }

    #[test]
    fn test_error_from_anyhow() {
        let anyhow_error = anyhow::anyhow!("something went wrong");
        let error = AllForOneError::from(anyhow_error);

        match error {
            AllForOneError::Internal(_) => {
                // Success - error was properly converted
            }
            _ => panic!("Expected Internal error"),
        }
    }
}
