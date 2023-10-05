use std::sync::Arc;

use axum::{
    middleware::from_extractor_with_state,
    routing::{delete, get, post, put},
    Router,
};
use hyper::StatusCode;

use crate::{
    api::v1::controllers::{
        ClientAuthController, ClientController, RedirectController, SessionController,
        UserAuthController, UserController,
    },
    middlewares::guards::*,
    oauth2::v1::controllers::{
        AuthorizeController, DeviceAuthorizationController, TokenController,
    },
    AppState,
};

pub fn routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/health_check", get(|| async { StatusCode::OK }))
        // -------------------------------------- OAUTH2 ROUTES ------------------------------------
        .nest(
            "/oauth2/v1",
            Router::new()
                .route("/authorize", post(AuthorizeController::handle))
                .route(
                    "/device_authorization",
                    post(DeviceAuthorizationController::handle),
                )
                .route("/token", post(TokenController::handle)),
        )
        // --------------------------------------   API ROUTES  ------------------------------------
        .nest(
            "/api/v1",
            Router::new()
                .nest(
                    "/clients",
                    Router::new()
                        .route("/:client_id", get(ClientController::read))
                        .route("/:client_id", put(ClientController::update))
                        .route("/:client_id", delete(ClientController::delete))
                        .route("/:client_id/redirects", get(RedirectController::read_all))
                        .layer(from_extractor_with_state::<ClientAuthGuard, AppState>(
                            state.clone(),
                        ))
                        .route("/", post(ClientAuthController::register)),
                )
                .nest(
                    "/redirects",
                    Router::new()
                        .route("/:redirect_id", get(RedirectController::read))
                        .route("/:redirect_id", delete(RedirectController::delete))
                        .layer(from_extractor_with_state::<RedirectAuthGuard, AppState>(
                            state.clone(),
                        ))
                        .route("/", post(RedirectController::create)),
                )
                .nest(
                    "/users",
                    Router::new()
                        .route("/:user_id", get(UserController::read))
                        .route("/:user_id", put(UserController::update))
                        .route("/:user_id", delete(UserController::delete))
                        .route("/:user_id/clients", get(ClientController::read_all))
                        .layer(from_extractor_with_state::<UserAuthGuard, AppState>(
                            state.clone(),
                        )),
                )
                .nest(
                    "/sessions",
                    Router::new()
                        .route("/:session_id", get(SessionController::read))
                        .route("/:session_id", put(SessionController::update))
                        .route("/:session_id", delete(SessionController::delete))
                        .layer(from_extractor_with_state::<SessionAuthGuard, AppState>(
                            state.clone(),
                        ))
                        .route("/me", get(SessionController::read_active))
                        .route("/", post(SessionController::create)),
                )
                .nest(
                    "/auth",
                    Router::new()
                        .route("/register", post(UserAuthController::register))
                        .route("/login", post(UserAuthController::authenticate)),
                ),
        )
}
