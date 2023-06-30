mod request_id;

use axum::{
    error_handling::HandleErrorLayer,
    http::{
        header::{HeaderName, ACCEPT, ACCEPT_ENCODING, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, StatusCode,
    },
    BoxError, Router,
};
use std::time::Duration;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, timeout::TimeoutLayer, ServiceBuilder};
use tower_http::{
    cors::CorsLayer,
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use request_id::RequestId;

pub fn with_middleware_stack(service: Router) -> Router {
    // security
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        .allow_headers([ACCEPT, ACCEPT_ENCODING, AUTHORIZATION, CONTENT_TYPE])
        .allow_origin([
            "http://127.0.0.1:8080".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:8081".parse::<HeaderValue>().unwrap(),
        ])
        .allow_credentials(true);

    let rate_limit_layer = ServiceBuilder::new()
        // this middleware goes above `GovernorLayer` because it will receive
        // errors returned by `GovernorLayer`
        .layer(HandleErrorLayer::new(|e: BoxError| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled error: {}", e),
            )
        }))
        .layer(BufferLayer::new(1024))
        .layer(RateLimitLayer::new(5, Duration::from_secs(1)));

    let timeout_layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: BoxError| async move {
            if e.is::<tower::timeout::error::Elapsed>() {
                (StatusCode::REQUEST_TIMEOUT, e.to_string())
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }))
        .layer(TimeoutLayer::new(Duration::from_secs(10)));

    // logging
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let x_request_id = HeaderName::from_static("x-request-id");

    let request_id_layer = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(
            x_request_id.clone(),
            RequestId::default(),
        ))
        .layer(PropagateRequestIdLayer::new(x_request_id));

    service
        .layer(trace_layer)
        .layer(request_id_layer)
        .layer(timeout_layer)
        .layer(rate_limit_layer)
        .layer(cors_layer)
}
