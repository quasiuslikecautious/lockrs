mod api;
mod common;
mod db;
mod middlewares;
mod oauth2;
mod routes;

pub use self::common::*;

use std::net::SocketAddr;

/// rfc: https://www.rfc-editor.org/rfc/rfc6749#section-4
#[tokio::main]
async fn main() {
    let state = AppState::new().await;

    let app = routes::routes().with_state(state);
    let app = middlewares::with_middleware_stack(app);

    // run it with hyper on localhost:9000
    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    tracing::info!("listening at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
