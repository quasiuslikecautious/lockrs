mod request_id;

use std::time::Duration;

use axum::{
    body::Body,
    error_handling::HandleErrorLayer,
    http::{
        header::{HeaderName, ACCEPT, ACCEPT_ENCODING, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, Request, Response, StatusCode,
    },
    BoxError, Router,
};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, timeout::TimeoutLayer, ServiceBuilder};
use tower_http::{
    cors::CorsLayer,
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing::Span;
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

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
    let filter = Targets::new()
        .with_target("server", tracing::Level::TRACE)
        .with_target("lockrs::trace::http", tracing::Level::TRACE)
        .with_default(tracing::Level::INFO);

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
            let x_request_id = &request.headers()["x-request-id"];

            tracing::debug_span!(
                target: "lockrs::trace::http",
                "http-request",
                "x-request-id" = ?x_request_id
            )
        })
        .on_request(|request: &Request<Body>, _span: &Span| {
            tracing::debug!(
                target: "lockrs::trace::http",
                "started processing request {} {} -- {:?}",
                request.method(),
                request.uri().path(),
                request.headers()
            )
        })
        .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
            tracing::debug!(
                target: "lockrs::trace::http",
                "finished processing request in {} ms -- {}",
                latency.as_millis(),
                response.status()
            )
        });

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
