use axum::{routing::post, Router};

use crate::auth::controllers::AuthController;

pub fn routes() -> Router {
    Router::new().route("/", post(AuthController::auth))
}
