use std::sync::Arc;

use anyhow::{Context, Result};
use oauth2::{
    AuthUrl, Client, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope,
    TokenResponse, TokenUrl, basic::BasicClient,
};
use openidconnect::{ClientId, ClientSecret};
use reqwest::redirect::Policy;
use url::Url;

use crate::{
    provider::types::{
        config::{AuthRedirectInfo, Authentication, OAuthClientConfig},
        idp_uid::GithubUid,
    },
    utils::types::HTTP_REQUEST_USER_AGENT,
};

pub type GithubClient = Arc<
    Client<
        oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
        oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
        oauth2::StandardTokenIntrospectionResponse<
            oauth2::EmptyExtraTokenFields,
            oauth2::basic::BasicTokenType,
        >,
        oauth2::StandardRevocableToken,
        oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
        oauth2::EndpointSet,
        oauth2::EndpointNotSet,
        oauth2::EndpointNotSet,
        oauth2::EndpointNotSet,
        oauth2::EndpointSet,
    >,
>;

#[derive(Debug, Clone)]
pub struct GithubAuthenticator {
    github_client: GithubClient,
    http_client: reqwest::Client,
    resource_url: Url,
}

impl GithubAuthenticator {
    pub fn new(config: OAuthClientConfig) -> Result<Self> {
        let resource_url =
            Url::parse(&config.resource_url).context("fail to parse resource url")?;
        let github_client =
            github_config_client(config).context("failed to create github client")?;
        let http_client = reqwest::Client::builder()
            .redirect(Policy::none())
            .build()
            .context("fail to make github http client")?;

        Ok(GithubAuthenticator {
            github_client,
            resource_url,
            http_client,
        })
    }
}

impl GithubAuthenticator {
    pub async fn get_user_info(&self, access_token: String) -> Result<String> {
        let user_info_url = self.resource_url.join("user")?;
        let response = self
            .http_client
            .get(user_info_url)
            .bearer_auth(access_token.clone())
            .header(
                reqwest::header::USER_AGENT,
                HTTP_REQUEST_USER_AGENT.get().unwrap(),
            )
            .send()
            .await
            .context("failed to send request to user info endpoint")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to fetch user info: {}",
                response.status()
            ));
        }

        let user = response
            .json::<GithubUid>()
            .await
            .context("fail to get user uid")?;

        Ok(user.id.to_string())
    }
}

impl Authentication for GithubAuthenticator {
    async fn auth_redirect_info(&self) -> AuthRedirectInfo {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let (auth_url, csrf_token) = self
            .github_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("read:user".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        AuthRedirectInfo {
            auth_url: auth_url.to_string(),
            csrf_token: csrf_token.secret().to_string(),
            pkce_verifier: pkce_verifier.secret().to_string(),
            nonce: None,
        }
    }

    async fn callback(&self, authorization_code: String, pkce_verifier: String) -> Result<String> {
        let resp = self
            .github_client
            .exchange_code(oauth2::AuthorizationCode::new(authorization_code))
            .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
            .request_async(&self.http_client)
            .await
            .context("fail to verify")?;

        Ok(resp.access_token().secret().to_owned())
    }
}

fn github_config_client(config: OAuthClientConfig) -> Result<GithubClient> {
    let idp_secret = ClientSecret::new(config.client_secret);
    let idp_id = ClientId::new(config.client_id);
    let auth_url = AuthUrl::new(config.auth_url).context("Invalid authorization endpoint URL")?;
    let token_url = TokenUrl::new(config.token_url).context("Invalid token endpoint URL")?;

    let github_oauth_config_client = BasicClient::new(idp_id)
        .set_client_secret(idp_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(RedirectUrl::from_url(config.redirect_url));
    Ok(Arc::new(github_oauth_config_client))
}
