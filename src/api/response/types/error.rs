use sonic_rs::Serialize;

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub status_code: u16,
    pub details: Option<Vec<ErrorResponseDetails>>,
}

#[derive(Serialize, Debug)]
pub struct ErrorResponseDetails {
    pub field: String,
    pub message: String,
}

pub const INVALID_VALUE: &str = "INVALID_VALUE";
pub const SERVER_ERROR: &str = "SERVER_ERROR";
