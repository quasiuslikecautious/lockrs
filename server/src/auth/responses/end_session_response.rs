use axum::{
    http::{header::SET_COOKIE, StatusCode},
    response::{AppendHeaders, IntoResponse},
};
use cookie::{Cookie, SameSite};
use serde::Serialize;






#[derive(Debug, Serialize)]
pub struct EndSessionResponse {}

impl EndSessionResponse {
    pub fn clear_cookie(name: &str) -> String {
        Cookie::build(name, "")
            .path("/")
            .http_only(true)
            .same_site(SameSite::Strict)
            .finish()
            .to_string()
    }
}

impl IntoResponse for EndSessionResponse {
    fn into_response(self) -> axum::response::Response {
        (
            AppendHeaders([
                (SET_COOKIE, Self::clear_cookie("s_jwt").as_str()),
                (SET_COOKIE, Self::clear_cookie("s_id").as_str()),
                (SET_COOKIE, Self::clear_cookie("u_id").as_str()),
            ]),
            StatusCode::NO_CONTENT,
        )
            .into_response()
    }
}
