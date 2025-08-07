use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};

#[derive(thiserror::Error, Debug)]
pub enum AllForOneError {
    #[error("path extraction error")]
    Path(#[from] PathRejection),
    #[error("query extraction error")]
    Query(#[from] QueryRejection),
    #[error("json extraction error")]
    Json(#[from] JsonRejection),

    #[error("auth error")]
    Auth(String),

    #[error("database error")]
    Db(#[from] sea_orm::DbErr),

    #[error("internal error")]
    Internal(#[from] anyhow::Error),
}
