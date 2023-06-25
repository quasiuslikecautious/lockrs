use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct RefreshTokenCreateModel {
    pub access_token_id: i32,
    pub token: String,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub expires_at: NaiveDateTime,
    pub scopes: Vec<String>,
}
