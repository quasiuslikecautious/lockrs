mod api;
mod common;
mod db;
mod middlewares;
mod oauth2;
mod routes;

pub use self::common::*;

use std::{net::SocketAddr, sync::Arc};

/// rfc: https://www.rfc-editor.org/rfc/rfc6749#section-4
#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new().await);
    let app_routes = routes::routes(state);
    let app = middlewares::with_middleware_stack(app_routes);

    // run it with hyper on localhost:8080
    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    println!("listening at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
