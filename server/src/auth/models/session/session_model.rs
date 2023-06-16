use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionModel {
    pub id: String,
    pub user_id: Uuid,
    pub expires_at: i64,
}

impl SessionModel {
    pub fn new(id: &str, user_id: &Uuid, expires_at: &i64) -> Self {
        Self {
            id: id.to_string(),
            user_id: *user_id,
            expires_at: *expires_at,
        }
    }
}
