use axum::{response::IntoResponse, Json};
use serde::Serialize;
use url::Url;
use uuid::Uuid;

#[derive(Serialize)]
pub struct RedirectResponse {
    pub id: Uuid,
    pub client_id: String,
    pub uri: Url,
}

impl IntoResponse for RedirectResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
