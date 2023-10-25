use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionResponse {
    pub id: String,
    pub user_id: Uuid,
    pub expires_at: i64,
}

impl IntoResponse for SessionResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
