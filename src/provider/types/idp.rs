use sonic_rs::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OAuthProvider {
    Github,
}

impl OAuthProvider {
    pub fn as_str(&self) -> &str {
        match self {
            OAuthProvider::Github => "github",
        }
    }
}
