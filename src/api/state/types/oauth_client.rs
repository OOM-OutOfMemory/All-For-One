use std::sync::Arc;

use crate::{
    config::types::Config,
    provider::{
        github::GithubAuthenticator,
        types::{AuthRedirectInfo, Authentication, OAuthClientConfig, OAuthProvider},
    },
};
use anyhow::{Context, Result};
use url::Url;

#[derive(Clone)]
pub struct OAuthProviderClient {
    http_client: reqwest::Client,
    pub github: Arc<GithubAuthenticator>,
}

impl OAuthProviderClient {
    pub fn new(config: &Config) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            // .redirect(reqwest::redirect::Policy::none())
            .build()
            .context("fail to build http client for redirect to callback path")?;

        let redirect_url_str = "/api/v1/oauth/github/callback";
        let mut base_url = Url::parse(&config.server.domain).context("fail to parse domain url")?;
        base_url.set_port(Some(config.server.port)).unwrap();

        let mut github_redirect_url = base_url.clone();
        github_redirect_url.set_path(redirect_url_str);

        let github_config = OAuthClientConfig {
            client_id: config.oidc.github.client_id.clone(),
            client_secret: config.oidc.github.client_secret.clone(),
            auth_url: config.oidc.github.auth_url.clone(),
            token_url: config.oidc.github.token_url.clone(),
            resource_url: config.oidc.github.resource_url.clone(),
            redirect_url: github_redirect_url,
        };

        Ok(OAuthProviderClient {
            http_client,
            github: Arc::new(GithubAuthenticator::new(github_config)?),
        })
    }

    pub async fn auth_request(&self, idp: OAuthProvider) -> AuthRedirectInfo {
        match idp {
            OAuthProvider::Github => self.github.clone(),
        }
        .auth_redirect_info()
        .await
    }

    pub async fn callback(
        &self,
        idp: OAuthProvider,
        authorization_code: String,
        pkce_verifier: String,
    ) -> Result<String> {
        match idp {
            OAuthProvider::Github => self.github.clone(),
        }
        .callback(authorization_code, pkce_verifier)
        .await
    }
}
