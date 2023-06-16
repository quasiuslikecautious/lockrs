mod auth;
mod oauth2;
mod shared;

pub use self::shared::*;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    body::{boxed, Body},
    http::{Response, StatusCode},
    response::IntoResponse,
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
        .with_default(tracing::Level::INFO);

    let tracing_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(tracing_layer)
        .with(filter)
        .init();

    let auth_routes = auth::routes::routes().with_state(Arc::new(AppState::new()));
    let oauth2_routes = oauth2::routes::routes();

    let app = Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .merge(auth_routes)
                .nest("/oauth2", oauth2_routes),
        )
        .fallback_service(get(|req| async move {
            match ServeDir::new(String::from("./dist")).oneshot(req).await {
                Ok(res) => res.map(boxed),
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {}", err))))
                    .expect("error response"),
            }
        }))
        .layer(TraceLayer::new_for_http());

    // run it with hyper on localhost:8080
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    println!("listening at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
