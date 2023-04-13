use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ClientResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub homepage_url: String,
}

impl IntoResponse for ClientResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
