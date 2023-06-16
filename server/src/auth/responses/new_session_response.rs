use axum::{
    http::{header::SET_COOKIE, StatusCode},
    response::{AppendHeaders, IntoResponse},
    Json,
};
use serde::Serialize;
use uuid::Uuid;

use std::sync::Arc;

use crate::utils::jwt::JwtUtil;

#[derive(Debug, Serialize)]
pub struct NewSessionResponse {
    #[serde(skip)]
    pub jwt_util: Arc<JwtUtil>,

    pub id: String,
    pub user_id: Uuid,
    pub expires_at: i64,
}

impl IntoResponse for NewSessionResponse {
    fn into_response(self) -> axum::response::Response {
        let response = Arc::as_ref(&self.jwt_util).sign_jwt(&self);
        if response.is_err() {
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }

        let jwt = response.unwrap();

        (
            AppendHeaders([(SET_COOKIE, format!("sid={}", jwt).as_str())]),
            Json(self),
        )
            .into_response()
    }
}
