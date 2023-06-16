use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::AppState;

pub struct RedirectController;

impl RedirectController {
    pub async fn read_all(
        State(_state): State<Arc<AppState>>,
        Path(client_id): Path<String>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/clients/{}/redirects", client_id),
        )
    }

    pub async fn create(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
        (StatusCode::NOT_IMPLEMENTED, "/redirects".to_string())
    }

    pub async fn read(
        State(_state): State<Arc<AppState>>,
        Path(redirect_id): Path<String>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/redirects/{}", redirect_id),
        )
    }

    pub async fn update(
        State(_state): State<Arc<AppState>>,
        Path(redirect_id): Path<String>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/redirects/{}", redirect_id),
        )
    }

    pub async fn delete(
        State(_state): State<Arc<AppState>>,
        Path(redirect_id): Path<String>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/redirects/{}", redirect_id),
        )
    }
}
