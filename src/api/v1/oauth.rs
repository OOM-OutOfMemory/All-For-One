use std::sync::Arc;

use axum::{
    Router,
    extract::{
        Path, Query, State,
        rejection::{PathRejection, QueryRejection},
    },
    response::{IntoResponse, Redirect, Response},
    routing::get,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use deadpool::managed::Pool;
use deadpool_memcached::Manager;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    api::{
        state::types::{app::AppState, oauth_client::OAuthProviderClient},
        types::cookie::COOKIE_AUTH_REQUEST_ID,
    },
    memcached::{
        repo::{
            cache_auth_redirect_info_by_session_id,
            get_auth_redirect_info_from_memecached_by_session_id,
        },
        types::AuthVerifyToken,
    },
    provider::types::{AuthRedirectInfo, OAuthProvider},
    utils::error::AllForOneError,
};

pub async fn oauth_login(
    path: Result<Path<OAuthProvider>, PathRejection>,
    State(oauth_client): State<Arc<OAuthProviderClient>>,
    State(memcached_client): State<Arc<Pool<Manager>>>,
    jar: CookieJar,
) -> Result<Response, AllForOneError> {
    let Path(idp) = path?;

    let AuthRedirectInfo {
        auth_url,
        csrf_token,
        pkce_verifier,
        nonce,
    } = oauth_client.auth_request(idp).await;
    let cache_body = AuthVerifyToken {
        csrf_token,
        pkce_verifier,
        nonce,
    };

    let session_id = Uuid::now_v7();

    cache_auth_redirect_info_by_session_id(memcached_client, session_id, &cache_body).await?;

    let session_cookie = Cookie::build((COOKIE_AUTH_REQUEST_ID, session_id.to_string()))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .max_age(cookie::time::Duration::seconds(12))
        .path("/")
        .build();
    let updated_jar = jar.add(session_cookie);

    Ok((updated_jar, Redirect::to(&auth_url)).into_response())
}

#[derive(Deserialize, Debug)]
struct OAuthCallbackQuery {
    pub code: String,
    pub state: String,
}

async fn oauth_callback(
    query: Result<Query<OAuthCallbackQuery>, QueryRejection>,
    path: Result<Path<OAuthProvider>, PathRejection>,
    State(oauth_client): State<Arc<OAuthProviderClient>>,
    State(memcached_client): State<Arc<Pool<Manager>>>,
    jar: CookieJar,
) -> Result<Response, AllForOneError> {
    let Path(idp) = path?;
    let Query(callback_params) = query?;
    let session_id = jar
        .get(COOKIE_AUTH_REQUEST_ID)
        .and_then(|cookie| cookie.value().parse::<Uuid>().ok())
        .ok_or_else(|| AllForOneError::Auth("session id is not found".to_string()))?;

    let verification_token =
        get_auth_redirect_info_from_memecached_by_session_id(memcached_client, session_id).await?;

    if verification_token.csrf_token != callback_params.state {
        return Err(AllForOneError::Auth("csrf token is invalid".to_string()));
    }

    let auth_request = oauth_client
        .callback(idp, callback_params.code, verification_token.pkce_verifier)
        .await?;

    let session_remove = Cookie::build((COOKIE_AUTH_REQUEST_ID, ""))
        .path("/")
        .build();
    let updated_jar = jar.remove(session_remove);

    Ok((
        updated_jar,
        format!(
            "oauth callback and will return jwt; access_token = {}",
            auth_request
        ),
    )
        .into_response())
}

pub async fn router(app_state: AppState) -> Router {
    axum::Router::new()
        .route("/{idp}/login", get(oauth_login))
        .route("/{idp}/callback", get(oauth_callback))
        .with_state(app_state)
}
