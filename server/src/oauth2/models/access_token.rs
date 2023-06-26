use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct AccessTokenModel {
    pub id: i32,
    pub token: String,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub expires_at: NaiveDateTime,
    pub scopes: Vec<String>,
}

pub struct AccessTokenCreateModel {
    pub token: String,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub expires_at: NaiveDateTime,
    pub scopes: Vec<String>,
}
