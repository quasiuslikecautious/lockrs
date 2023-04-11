use axum::Router;

mod user_routes;
mod session_routes;
mod client_routes;
mod redirect_routes;

pub fn routes() -> Router {
    Router::new()
        .nest("/users", user_routes::routes())
        .nest("/sessions", session_routes::routes())
}

