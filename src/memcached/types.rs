use sonic_rs::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthVerifyToken {
    pub csrf_token: String,
    pub pkce_verifier: String,
    pub nonce: Option<String>,
}
