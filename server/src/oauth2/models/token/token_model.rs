pub struct TokenModel {
    pub token_type: String,
    pub expires_in: i64,
    pub access_token: String,
    pub refresh_token: String,
    pub scopes: String,
}
