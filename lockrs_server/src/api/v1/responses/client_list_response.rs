use axum::{response::IntoResponse, Json};
use serde::Serialize;

use super::ClientResponse;

#[derive(Serialize)]
pub struct ClientListResponse {
    pub clients: Vec<ClientResponse>,
}

impl IntoResponse for ClientListResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
