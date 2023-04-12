use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::auth::controllers::RedirectController;

pub fn client_routes() -> Router {
    Router::new()
        .route("/", get(RedirectController::read_all))
        .route("/", post(RedirectController::create))
        .route("/:redirect_id", get(RedirectController::read))
        .route("/:redirect_id", put(RedirectController::update))
        .route("/:redirect_id", delete(RedirectController::delete))
}
