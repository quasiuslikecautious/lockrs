use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{auth::controllers::AuthController, AppState};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", post(AuthController::auth))
}
