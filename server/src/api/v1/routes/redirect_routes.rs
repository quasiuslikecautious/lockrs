use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{api::v1::controllers::RedirectController, AppState};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(RedirectController::create))
        .route("/:redirect_id", get(RedirectController::read))
        .route("/:redirect_id", put(RedirectController::update))
        .route("/:redirect_id", delete(RedirectController::delete))
}

pub fn client_routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(RedirectController::read_all))
}
