use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct RefreshToken {
    pub token: String,
    pub user_id: Option<Uuid>,
    pub client_id: String,
    pub scopes: Vec<String>,
    pub expires_at: NaiveDateTime,
}

