use sonic_rs::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub aud: String,
    pub iss: String,
    pub sub: Uuid,
    pub exp: i64,
    pub jti: Uuid,
    pub iat: i64,
    pub nbf: i64,
}
