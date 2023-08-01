use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::COOKIE, request::Parts, StatusCode},
    response::IntoResponse,
};

use std::collections::HashMap;

#[derive(Debug)]
pub struct Cookies(pub HashMap<String, String>);

#[async_trait()]
impl<S> FromRequestParts<S> for Cookies
where
    S: Send + Sync,
{
    type Rejection = CookieError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;

        let Some(cookie_header) = headers.get(COOKIE)
        else {
            return Err(Self::Rejection::InvalidHeader);
        };

        let Some(cookie_str) = cookie_header.to_str().ok()
        else {
            return Err(Self::Rejection::InvalidHeaderEncoding);
        };

        let cookies = cookie_str
            .split(';')
            .map(|pair| pair.splitn(2, '=').collect::<Vec<&str>>())
            .filter(|pair| pair.len() == 2)
            .map(|pair| (pair[0].to_string(), pair[1].to_string()))
            .collect::<HashMap<String, String>>();

        Ok(Cookies(cookies))
    }
}

pub enum CookieError {
    InvalidHeader,
    InvalidHeaderEncoding,
}

impl IntoResponse for CookieError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::BAD_REQUEST.into_response()
    }
}
