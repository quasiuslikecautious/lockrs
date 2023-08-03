use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use headers::{authorization::Bearer, Authorization, HeaderMapExt};

pub async fn guard<T>(request: Request<T>, _next: Next<T>) -> Result<Response, StatusCode> {
    let _token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(StatusCode::BAD_REQUEST)?
        .token()
        .to_owned();

    todo!();
}
