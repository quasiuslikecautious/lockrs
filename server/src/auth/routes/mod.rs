use axum::Router;

mod client_routes;
mod redirect_routes;
mod session_routes;
mod user_routes;

pub fn routes() -> Router {
    Router::new()
        .nest("/users", user_routes::routes())
        .nest("/sessions", session_routes::routes())
}
