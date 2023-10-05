use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
}

impl IntoResponse for UserResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
