use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use uuid::Uuid;

pub struct RedirectController;

impl RedirectController {
    pub async fn read_all(Path((user_id, client_id)): Path<(Uuid, String)>) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/users/{}/clients/{}/redirects", user_id, client_id),
        )
    }

    pub async fn create(Path((user_id, client_id)): Path<(Uuid, String)>) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/users/{}/clients/{}/redirects", user_id, client_id),
        )
    }

    pub async fn read(
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
