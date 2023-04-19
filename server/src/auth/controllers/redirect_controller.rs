use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};

use crate::AppState;

pub struct RedirectController;

impl RedirectController {
    pub async fn read_all(
        Extension(_state): Extension<Arc<AppState>>,
        Path(client_id): Path<String>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/clients/{}/redirects", client_id),
        )
    }

    pub async fn create(Extension(_state): Extension<Arc<AppState>>) -> impl IntoResponse {
        (StatusCode::NOT_IMPLEMENTED, "/redirects".to_string())
    }

    pub async fn read(
        Extension(_state): Extension<Arc<AppState>>,
        Path(redirect_id): Path<String>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/redirects/{}", redirect_id),
        )
    }

    pub async fn update(
        Extension(_state): Extension<Arc<AppState>>,
        Path(redirect_id): Path<String>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/redirects/{}", redirect_id),
        )
    }

    pub async fn delete(
        Extension(_state): Extension<Arc<AppState>>,
        Path(redirect_id): Path<String>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/redirects/{}", redirect_id),
        )
    }
}
