use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct AccessTokenModel {
    pub token: String,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub scopes: Vec<String>,
    pub expires_at: NaiveDateTime,
}
