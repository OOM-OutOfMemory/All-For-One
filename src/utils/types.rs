use std::sync::OnceLock;

pub static HTTP_REQUEST_USER_AGENT: OnceLock<String> = OnceLock::new();
