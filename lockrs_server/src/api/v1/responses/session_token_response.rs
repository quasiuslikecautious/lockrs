use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionTokenResponse {
    pub session_token: String,
    pub expires_at: i64,
}

impl IntoResponse for SessionTokenResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
