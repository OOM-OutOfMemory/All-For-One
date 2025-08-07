use anyhow::Result;
use url::Url;

pub struct OIDCClientConfig {
    pub client_id: String,
    pub client_secret: String,
    pub issuer_url: String,
    pub redirect_uri: Url,
}

pub struct OAuthClientConfig {
    pub client_id: String,
    pub client_secret: String,
    pub resource_url: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: Url,
}

#[derive(Debug)]
pub struct AuthRedirectInfo {
    pub auth_url: String,
    pub csrf_token: String,
    pub pkce_verifier: String,
    pub nonce: Option<String>,
}

pub trait Authentication {
    async fn auth_redirect_info(&self) -> AuthRedirectInfo;
    async fn callback(&self, authorization_code: String, pkce_verifier: String) -> Result<String>;
}
