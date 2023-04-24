use axum::{Router, routing::post};

use crate::auth::controllers::AuthController;

pub fn routes() -> Router {
    Router::new()
        .route("/", post(AuthController::auth))
}
