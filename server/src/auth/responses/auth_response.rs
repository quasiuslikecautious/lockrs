use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct AuthResponse {
    pub session_token: String,
}

impl IntoResponse for AuthResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
