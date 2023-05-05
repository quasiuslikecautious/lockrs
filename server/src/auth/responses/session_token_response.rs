use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SessionTokenResponse {
    pub token: String,
}

impl IntoResponse for SessionTokenResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
