use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub id: String,
    pub token: String,
}

impl IntoResponse for SessionResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
