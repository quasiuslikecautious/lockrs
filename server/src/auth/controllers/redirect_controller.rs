use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};
use uuid::Uuid;

use crate::AppState;

pub struct RedirectController;

impl RedirectController {
    pub async fn read_all(
        Extension(_state): Extension<Arc<AppState>>,
        Path((user_id, client_id)): Path<(Uuid, String)>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/users/{}/clients/{}/redirects", user_id, client_id),
        )
    }

    pub async fn create(
        Extension(_state): Extension<Arc<AppState>>,
        Path((user_id, client_id)): Path<(Uuid, String)>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/users/{}/clients/{}/redirects", user_id, client_id),
        )
    }

    pub async fn read(
        Extension(_state): Extension<Arc<AppState>>,
        Path((user_id, client_id, redirect_id)): Path<(Uuid, String, String)>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!(
                "/users/{}/clients/{}/redirects/{}",
                user_id, client_id, redirect_id
            ),
        )
    }

    pub async fn update(
        Extension(_state): Extension<Arc<AppState>>,
        Path((user_id, client_id, redirect_id)): Path<(Uuid, String, String)>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!(
                "/users/{}/clients/{}/redirects/{}",
                user_id, client_id, redirect_id
            ),
        )
    }

    pub async fn delete(
        Extension(_state): Extension<Arc<AppState>>,
        Path((user_id, client_id, redirect_id)): Path<(Uuid, String, String)>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!(
                "/users/{}/clients/{}/redirects/{}",
                user_id, client_id, redirect_id
            ),
        )
    }
}
