use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};

#[derive(thiserror::Error, Debug)]
pub enum AllForOneError {
    #[error("path extraction error")]
    Path(#[from] PathRejection),
    #[error("query extraction error")]
    Query(#[from] QueryRejection),
    #[error("json extraction error")]
    Json(#[from] JsonRejection),
    #[error("header extraction error")]
    Header(String),

    #[error("parse error")]
    Parse(AllForOneParseError),

    #[error("auth error")]
    Auth(String),

    #[error("internal error")]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct AllForOneParseError {
    pub field: String,
    pub message: String,
}
