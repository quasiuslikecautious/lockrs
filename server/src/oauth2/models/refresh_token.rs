use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct RefreshTokenModel {
    pub id: i32,
    pub access_token_id: i32,
    pub token: String,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub scopes: Vec<String>,
    pub expires_at: NaiveDateTime,
}

pub struct RefreshTokenCreateModel {
    pub access_token_id: i32,
    pub token: String,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub expires_at: NaiveDateTime,
    pub scopes: Vec<String>,
}
