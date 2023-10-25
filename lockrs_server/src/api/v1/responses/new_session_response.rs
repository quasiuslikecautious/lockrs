use axum::{
    http::{header::SET_COOKIE, StatusCode},
    response::{AppendHeaders, IntoResponse},
    Json,
};
use cookie::{Cookie, SameSite};
use serde::Serialize;
use uuid::Uuid;

use std::sync::Arc;

use crate::{api::v1::models::SessionModel, utils::jwt::JwtUtil};

#[derive(Debug, Serialize)]
pub struct NewSessionResponse {
    #[serde(skip)]
    pub jwt_util: Arc<JwtUtil>,

    pub id: String,
    pub user_id: Uuid,
    pub expires_at: i64,
}

impl NewSessionResponse {
    pub fn new(jwt_util: &Arc<JwtUtil>, session: &SessionModel) -> Self {
        Self {
            jwt_util: jwt_util.clone(),
            id: session.id.clone(),
            user_id: session.user_id,
            expires_at: session.expires_at,
        }
    }

    pub fn create_http_cookie<'c>(name: &'c str, value: &'c str) -> String {
        Cookie::build(name, value)
            .path("/")
            .same_site(SameSite::Strict)
            .finish()
            .to_string()
    }

    pub fn create_cookie<'c>(name: &'c str, value: &'c str) -> String {
        Cookie::build(name, value)
            .path("/")
            .same_site(SameSite::Strict)
            .finish()
            .to_string()
    }

    pub fn create_auth_cookie(&self) -> Result<String, ()> {
        let signed_jwt = Arc::as_ref(&self.jwt_util).sign_jwt(self).map_err(|_| ())?;

        let auth_cookie = Self::create_http_cookie(JwtUtil::cookie_name(), signed_jwt.as_str());

        Ok(auth_cookie)
    }
}

impl IntoResponse for NewSessionResponse {
    fn into_response(self) -> axum::response::Response {
        let auth_cookie = self
            .create_auth_cookie()
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR).into_response())
            .unwrap();

        (
            AppendHeaders([
                (SET_COOKIE, auth_cookie.as_str()),
                (
                    SET_COOKIE,
                    Self::create_cookie("s_id", self.id.as_str()).as_str(),
                ),
                (
                    SET_COOKIE,
                    Self::create_cookie("u_id", self.user_id.to_string().as_str()).as_str(),
                ),
            ]),
            Json(self),
        )
            .into_response()
    }
}
