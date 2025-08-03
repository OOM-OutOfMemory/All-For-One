use sonic_rs::Serialize;

#[derive(Serialize)]
pub struct Token {
    pub access_token: String,
}
