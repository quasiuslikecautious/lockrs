use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SessionModel {
    pub id: String,
    pub user_id: Uuid,
    pub expires_at: i64,
}

impl SessionModel {
    pub fn new(id: &str, user_id: &Uuid, expires_at: i64) -> Self {
        Self {
            id: id.to_owned(),
            user_id: user_id.to_owned(),
            expires_at,
        }
    }
}

pub struct SessionCreateModel {
    pub session_token: String,
}

impl SessionCreateModel {
    pub fn new(session_token: &str) -> Self {
        Self {
            session_token: session_token.to_owned(),
        }
    }
}

impl std::fmt::Debug for SessionCreateModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SessionCreateModel: {{ session_token: ******** }}")
    }
}

#[derive(Debug)]
pub struct SessionUpdateModel {
    pub refresh: bool,
}

impl SessionUpdateModel {
    pub fn new(refresh: bool) -> Self {
        Self {
            refresh,
        }
    }
}
