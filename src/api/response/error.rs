use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    api::response::types::error::{
        ErrorResponse, ErrorResponseDetails, INVALID_VALUE, SERVER_ERROR,
    },
    utils::error::AllForOneError,
};

impl IntoResponse for AllForOneError {
    fn into_response(self) -> Response {
        let message = self.to_string();
        let (status, body) = match self {
            AllForOneError::Path(rejection) => (
                rejection.status(),
                ErrorResponse {
                    code: INVALID_VALUE.to_string(),
                    message,
                    status_code: rejection.status().as_u16(),
                    details: Some(vec![ErrorResponseDetails {
                        field: "url/path".to_string(),
                        message: rejection.to_string(),
                    }]),
                },
            ),
            AllForOneError::Query(rejection) => (
                rejection.status(),
                ErrorResponse {
                    code: INVALID_VALUE.to_string(),
                    message,
                    status_code: rejection.status().as_u16(),
                    details: Some(vec![ErrorResponseDetails {
                        field: "url/query".to_string(),
                        message: rejection.to_string(),
                    }]),
                },
            ),
            AllForOneError::Json(rejection) => (
                rejection.status(),
                ErrorResponse {
                    code: INVALID_VALUE.to_string(),
                    message,
                    status_code: rejection.status().as_u16(),
                    details: Some(vec![ErrorResponseDetails {
                        field: "body/json".to_string(),
                        message: rejection.to_string(),
                    }]),
                },
            ),

            AllForOneError::Auth(err) => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    code: "AUTH_ERROR".to_string(),
                    message: err,
                    status_code: 401,
                    details: None,
                },
            ),
            AllForOneError::Db(err) => {
                tracing::error!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse {
                        code: SERVER_ERROR.to_string(),
                        message: err.to_string(),
                        status_code: 500,
                        details: None,
                    },
                )
            }
            AllForOneError::Internal(err) => {
                tracing::error!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse {
                        code: SERVER_ERROR.to_string(),
                        message: err.to_string(),
                        status_code: 500,
                        details: None,
                    },
                )
            }
        };

        (status, Json(body)).into_response()
    }
}
