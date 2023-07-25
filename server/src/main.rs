mod api;
mod common;
mod db;
mod middlewares;
mod oauth2;

pub use self::common::*;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    body::{boxed, Body},
    http::{Response, StatusCode},
    routing::get,
    Router,
};
use tower::ServiceExt;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

/// rfc: https://www.rfc-editor.org/rfc/rfc6749#section-4
#[tokio::main]
async fn main() {
    let filter = Targets::new()
        .with_target("tower_http::trace::on_response", tracing::Level::DEBUG)
        .with_target("tower_http::trace::on_request", tracing::Level::DEBUG)
        .with_target("tower_http::trace::make_span", tracing::Level::DEBUG)
        .with_target("lockrs::trace", tracing::Level::DEBUG)
        .with_default(tracing::Level::INFO);

    let tracing_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(tracing_layer)
        .with(filter)
        .init();

    let state = Arc::new(AppState::new().await);

    let auth_v1_routes = api::v1::routes().with_state(Arc::clone(&state));
    let oauth2_v1_routes = oauth2::v1::routes().with_state(Arc::clone(&state));

    let app_routes = Router::new()
        .nest("/api/v1", auth_v1_routes)
        .nest("/oauth2/v1", oauth2_v1_routes)
        .fallback_service(get(|req| async move {
            match ServeDir::new(String::from("./dist")).oneshot(req).await {
                Ok(res) => res.map(boxed),
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {}", err))))
                    .expect("error response"),
            }
        }));

    let app = middlewares::with_middleware_stack(app_routes).layer(TraceLayer::new_for_http());

    // run it with hyper on localhost:8080
    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    println!("listening at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
