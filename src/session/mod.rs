use sonic_rs::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SessionStruct {
    pub session_id: Uuid,
    pub crsf_token: Uuid,
}
