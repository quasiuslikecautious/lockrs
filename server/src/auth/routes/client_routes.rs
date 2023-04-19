use axum::{
    routing::{delete, get, post, put},
    Router,
};

use super::redirect_routes;
use crate::auth::controllers::ClientController;

pub fn routes() -> Router {
    Router::new()
        .route("/", post(ClientController::create))
        .route("/:client_id", get(ClientController::read))
        .route("/:client_id", put(ClientController::update))
        .route("/:client_id", delete(ClientController::delete))
        .nest("/:client_id/redirects", redirect_routes::client_routes())
}

pub fn user_routes() -> Router {
    Router::new().route("/", get(ClientController::read_all))
}
