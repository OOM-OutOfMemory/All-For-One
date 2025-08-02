use anyhow::{Context, Result};
use axum::extract::FromRef;
use deadpool::managed::Pool;
use deadpool_memcached::Manager;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use url::Url;

use crate::{
    config::types::Config,
    memcached::types::MemCachedClient,
    provider::{
        github::GithubAuthenticator,
        types::{AuthRedirectInfo, Authentication, OAuthClientConfig, OAuthProvider},
    },
};

#[derive(Clone)]
pub struct AppState {
    pub oauth_provider_state: Arc<OAuthProviderClient>,
    pub postgres_state: Arc<DatabaseConnection>,
    pub memcached_state: Arc<Pool<Manager>>,
}

#[derive(Clone)]
pub struct OAuthProviderClient {
    http_client: reqwest::Client,
    pub github: Arc<GithubAuthenticator>,
}

impl OAuthProviderClient {
    pub fn new(config: &Config) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
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
}

impl FromRef<AppState> for Arc<OAuthProviderClient> {
    fn from_ref(input: &AppState) -> Self {
        input.oauth_provider_state.clone()
    }
}

impl FromRef<AppState> for Arc<DatabaseConnection> {
    fn from_ref(input: &AppState) -> Self {
        input.postgres_state.clone()
    }
}

impl FromRef<AppState> for Arc<Pool<Manager>> {
    fn from_ref(input: &AppState) -> Self {
        input.memcached_state.clone()
    }
}
