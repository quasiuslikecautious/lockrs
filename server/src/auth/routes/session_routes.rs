use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::auth::controllers::SessionController;

pub fn routes() -> Router {
    Router::new()
        .route("/", post(SessionController::create))
        .route("/:session_id", get(SessionController::read))
        .route("/:session_id", put(SessionController::update))
        .route("/:session_id", delete(SessionController::delete))
}

pub fn user_routes() -> Router {
    Router::new().route("/", get(SessionController::read_all))
}
