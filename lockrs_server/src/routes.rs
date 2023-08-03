use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    api::v1::controllers::{
        ClientAuthController, ClientController, RedirectController, SessionController,
        UserAuthController, UserController,
    },
    oauth2::v1::controllers::{
        AuthorizeController, DeviceAuthorizationController, TokenController,
    },
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
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
                        .route("/", post(ClientAuthController::register))
                        .route("/:client_id", get(ClientController::read))
                        .route("/:client_id", put(ClientController::update))
                        .route("/:client_id", delete(ClientController::delete))
                        .route("/:client_id/redirects", get(RedirectController::read_all)),
                )
                .nest(
                    "/redirects",
                    Router::new()
                        .route("/", post(RedirectController::create))
                        .route("/:redirect_id", get(RedirectController::read))
                        .route("/:redirect_id", put(RedirectController::update))
                        .route("/:redirect_id", delete(RedirectController::delete)),
                )
                .nest(
                    "/sessions",
                    Router::new()
                        .route("/", post(SessionController::create))
                        .route("/:session_id", get(SessionController::read))
                        .route("/:session_id", put(SessionController::update))
                        .route("/:session_id", delete(SessionController::delete)),
                )
                .nest(
                    "/auth",
                    Router::new()
                        .route("/register", post(UserAuthController::register))
                        .route("/login", post(UserAuthController::authenticate)),
                )
                .nest(
                    "/users",
                    Router::new()
                        .route("/:user_id", get(UserController::read))
                        .route("/:user_id", put(UserController::update))
                        .route("/:user_id", delete(UserController::delete))
                        .route("/:user_id/clients", get(ClientController::read_all))
                        .route("/:user_id/sessions", get(SessionController::read_all)),
                ),
        )
}
