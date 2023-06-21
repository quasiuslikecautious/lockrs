use std::sync::Arc;

use axum::Router;

use crate::AppState;

mod auth_routes;
mod client_routes;
mod redirect_routes;
mod session_routes;
mod user_routes;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth_routes::routes())
        .nest("/clients", client_routes::routes())
        .nest("/redirect", redirect_routes::routes())
        .nest("/sessions", session_routes::routes())
        .nest("/users", user_routes::routes())
}
