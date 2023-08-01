use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{api::v1::controllers::UserAuthController, AppState};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", post(UserAuthController::auth))
}
