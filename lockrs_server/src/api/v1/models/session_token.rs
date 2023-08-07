use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct SessionTokenModel {
    pub token: String,
    pub user_id: Uuid,
    pub expires_at: i64,
}

impl SessionTokenModel {
    pub fn new(token: &str, user_id: &Uuid, expires_at: i64) -> Self {
        Self {
            token: token.to_owned(),
            user_id: user_id.to_owned(),
            expires_at,
        }
    }
}

impl std::fmt::Debug for SessionTokenModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SessionTokenModel: {{ token: ********, {:?}, {:?} }}",
            self.user_id, self.expires_at
        )
    }
}
