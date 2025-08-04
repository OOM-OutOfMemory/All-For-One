use std::sync::Arc;

use anyhow::{Context, Result};
use deadpool::managed::Pool;
use deadpool_memcached::Manager;
use uuid::Uuid;

use crate::memcached::types::AuthVerifyToken;

pub async fn cache_auth_redirect_info_by_session_id(
    client: Arc<Pool<Manager>>,
    session_id: Uuid,
    body: &AuthVerifyToken,
) -> Result<()> {
    let body = sonic_rs::json!(body);

    let result = client
        .get()
        .await
        .context("fail to get memcached client from pool")?
        .set(session_id.to_string(), body.to_string(), Some(10), None)
        .await
        .context("fail to cache auth infomation by session id")?;
    Ok(result)
}

pub async fn get_auth_redirect_info_from_memecached_by_session_id(
    client: Arc<Pool<Manager>>,
    session_id: Uuid,
) -> Result<AuthVerifyToken> {
    let result = client
        .get()
        .await
        .context("fail to get memcached client from pool")?
        .get(session_id.to_string())
        .await
        .context("fail to get auth redirect info by session id")?;

    match result {
        Some(value) => {
            let auth_info = sonic_rs::from_slice(&value.data)
                .context("fail to parse AuthRedirectInfo from memecached json")?;
            Ok(auth_info)
        }
        None => Err(anyhow::anyhow!("No auth redirect info found by session id")),
    }
}
