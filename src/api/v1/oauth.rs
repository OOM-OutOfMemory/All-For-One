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
use axum_extra::extract::CookieJar;
use deadpool::managed::Pool;
use deadpool_memcached::Manager;
use sea_orm::{DatabaseConnection, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    api::{
        state::types::{app::AppState, jwt_issuer, oauth_client::OAuthProviderClient},
        types::{cookie::COOKIE_AUTH_REQUEST_ID, session::SessionCookieConfig},
    },
    db::repo::users::UsersRepo,
    memcached::{
        repo::{
            cache_auth_redirect_info_by_session_id,
            get_auth_redirect_info_from_memecached_by_session_id,
        },
        types::AuthVerifyToken,
    },
    provider::types::{config::AuthRedirectInfo, idp::OAuthProvider},
    utils::error::AllForOneError,
};

pub async fn oauth_login(
    path: Result<Path<OAuthProvider>, PathRejection>,
    State(oauth_client): State<Arc<OAuthProviderClient>>,
    State(memcached_client): State<Arc<Pool<Manager>>>,
    State(session_config): State<Arc<SessionCookieConfig>>,
    jar: CookieJar,
) -> Result<Response, AllForOneError> {
    let Path(idp) = path?;

    let AuthRedirectInfo {
        auth_url,
        csrf_token,
        pkce_verifier,
        nonce,
    } = oauth_client.auth_request(idp).await;

    let session_id = Uuid::now_v7();
    let cache_body = AuthVerifyToken {
        csrf_token,
        pkce_verifier,
        nonce,
    };
    cache_auth_redirect_info_by_session_id(
        memcached_client,
        session_id,
        &cache_body,
        session_config.cache_ttl,
    )
    .await?;

    let session_cookie = session_config.create_session_cookie(&session_id.to_string());
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
    State(db_client): State<Arc<DatabaseConnection>>,
    State(jwt_issuer): State<Arc<jwt_issuer::JwtIssuer>>,
    State(session_config): State<Arc<SessionCookieConfig>>,
    jar: CookieJar,
) -> Result<Response, AllForOneError> {
    let Path(idp) = path?;
    let Query(callback_params) = query?;
    let session_id = jar
        .get(COOKIE_AUTH_REQUEST_ID)
        .and_then(|cookie| cookie.value().parse::<Uuid>().ok())
        .ok_or_else(|| AllForOneError::Auth("session id is not found".to_string()))?;

    let session_remove = session_config.create_session_removal_cookie();
    let updated_jar = jar.remove(session_remove);

    let verification_token =
        get_auth_redirect_info_from_memecached_by_session_id(memcached_client, session_id).await?;
    if verification_token.csrf_token != callback_params.state {
        return Err(AllForOneError::Auth("csrf token is invalid".to_string()));
    }

    let access_token = oauth_client
        .callback(
            idp.clone(),
            callback_params.code,
            verification_token.pkce_verifier,
        )
        .await?;

    let idp_uid = oauth_client
        .get_user_info(idp.clone(), access_token.clone())
        .await?;

    let txn = db_client.begin().await?;
    let user_repo = UsersRepo::new(&txn);
    let user = user_repo
        .get_or_create_user_if_not_exist(idp, idp_uid)
        .await?;
    txn.commit().await?;

    let key_id = jwt_issuer.get_kid();
    let access_token_ttl = jwt_issuer.get_access_token_ttl();
    let jwt = jwt_issuer
        .issue_jwt(key_id, user.id, access_token_ttl)
        .map_err(|e| AllForOneError::Auth(format!("fail to issue jwt: {}", e)))?;
    let response_body = crate::api::response::types::token::Token { access_token: jwt };

    Ok((
        updated_jar,
        (axum::http::StatusCode::OK, axum::Json(response_body)),
    )
        .into_response())
}

pub async fn router(app_state: AppState) -> Router {
    axum::Router::new()
        .route("/{idp}/login", get(oauth_login))
        .route("/{idp}/callback", get(oauth_callback))
        .with_state(app_state)
}
