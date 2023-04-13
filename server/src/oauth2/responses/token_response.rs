use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token_type: String, // usually just 'Bearer'
    pub expires_in: i64,
    pub access_token: String,  // 10 minutes
    pub refresh_token: String, // 24 hours
    pub scopes: String,
}

impl IntoResponse for TokenResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
