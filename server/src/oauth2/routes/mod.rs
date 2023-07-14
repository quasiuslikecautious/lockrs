use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{
    oauth2::controllers::{AuthorizeController, DeviceAuthorizationController, TokenController},
    AppState,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/authorize", post(AuthorizeController::handle))
        .route(
            "/device_authorization",
            post(DeviceAuthorizationController::handle),
        )
        .route("/token", post(TokenController::handle))
}
