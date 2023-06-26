use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct SessionTokenModel {
    pub token: String,
    pub user_id: Uuid,
    pub expires_at: i64,
}
