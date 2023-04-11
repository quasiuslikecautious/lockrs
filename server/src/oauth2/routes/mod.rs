use axum::{
    Router,
    routing::post,
};

use super::controllers::{AuthorizeController, DeviceAuthorizationController, TokenController};

pub fn routes() -> Router {
    Router::new()
        .route("/authorize", post(AuthorizeController::handle))
        .route("/device_authorization", post(DeviceAuthorizationController::handle))
        .route("/token", post(TokenController::handle))
}
