use uuid::Uuid;

pub struct TokenCreateModel {
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub scopes: Vec<String>,
}

pub struct TokenModel {
    pub token_type: String,
    pub expires_in: i64,
    pub access_token: String,
    pub refresh_token: String,
    pub scopes: String,
}
