use sonic_rs::Serialize;
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct AuthUrlResponse {
    pub auth_url: Url,
    pub csrf_token: Uuid,
}
