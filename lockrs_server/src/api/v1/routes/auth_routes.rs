use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{api::v1::controllers::AuthController, AppState};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", post(AuthController::auth))
}
