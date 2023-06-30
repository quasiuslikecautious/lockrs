use axum::{
    http::{header::SET_COOKIE, StatusCode},
    response::{AppendHeaders, IntoResponse},
    Json,
};
use cookie::{Cookie, SameSite};
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

impl NewSessionResponse {
    pub fn create_http_cookie<'c>(name: &'c str, value: &'c String) -> String {
        Cookie::build(name, value.as_str())
            .path("/")
            .same_site(SameSite::Strict)
            .finish()
            .to_string()
    }

    pub fn create_cookie<'c>(name: &'c str, value: &'c String) -> String {
        Cookie::build(name, value.as_str())
            .path("/")
            .same_site(SameSite::Strict)
            .finish()
            .to_string()
    }
}

impl IntoResponse for NewSessionResponse {
    fn into_response(self) -> axum::response::Response {
        let response = Arc::as_ref(&self.jwt_util).sign_jwt(&self);
        if response.is_err() {
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }

        let jwt = response.unwrap();

        (
            AppendHeaders([
                (SET_COOKIE, Self::create_http_cookie("s_jwt", &jwt).as_str()),
                (SET_COOKIE, Self::create_cookie("s_id", &self.id).as_str()),
                (
                    SET_COOKIE,
                    Self::create_cookie("u_id", &self.user_id.to_string()).as_str(),
                ),
            ]),
            Json(self),
        )
            .into_response()
    }
}
