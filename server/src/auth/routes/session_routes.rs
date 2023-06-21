use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{auth::controllers::SessionController, AppState};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(SessionController::create))
        .route("/:session_id", get(SessionController::read))
        .route("/:session_id", put(SessionController::update))
        .route("/:session_id", delete(SessionController::delete))
}

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(SessionController::read_all))
}
