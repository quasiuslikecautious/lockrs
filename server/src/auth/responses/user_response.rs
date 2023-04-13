use axum::{response::IntoResponse, Json};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
}

impl IntoResponse for UserResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
