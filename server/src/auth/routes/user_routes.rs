use axum::{
    routing::{delete, get, post, put},
    Router,
};

use super::{client_routes, session_routes};
use crate::auth::controllers::UserController;

pub fn routes() -> Router {
    Router::new()
        .route("/", post(UserController::create))
        .route("/:user_id", get(UserController::read))
        .route("/:user_id", put(UserController::update))
        .route("/:user_id", delete(UserController::delete))
        .nest("/:user_id/clients", client_routes::user_routes())
        .nest("/:user_id/sessions", session_routes::user_routes())
}
