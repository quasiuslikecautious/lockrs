mod auth_response;
mod models;
mod extractor;

use std::net::SocketAddr;

use axum::{
    extract::{ Json, Query },
    http::StatusCode,
    response::Redirect,
    Router,
    routing::{ get, post, },
};
use axum_macros::debug_handler;
use extractor::GrantType;
use models::{DeviceCodeParams, RefreshTokenParams};
use serde::{ Deserialize, Serialize, };
use url::Url;

/// TODO
/// Coordinate with database and assert that provided client_id exists and is valid
/// For now, assume this is set up and just return true
fn verify_client_id(client_id: &String) -> bool {
    return true;
}

/// TODO
/// Coordinate with database and assert that uri has been registered to the provided client_id
/// For now just assume this is set up and return true
fn is_redirect_registered(client_id: &String, redirect_uri: &Url) -> bool {
    return true;
}

/// TODO
/// Coordinate with database and assert that scopes exist and that the provided client_id has
/// access to use all of them 
/// For now just assume this is set up and return true
fn verify_scopes(client_id: &String, scopes: &Vec<String>) -> bool {
    return true;
}

/// rfc: https://www.rfc-editor.org/rfc/rfc6749#section-4
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/authorize", post(authorization_grant))
        .route("/token", post(token))
        .route("/error", get(auth_error))
        .fallback(fallback)
        .with_state(());

    // run it with hyper on localhost:8080
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening at {}", addr.to_string());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Fallback function - when a route is requested that doesn't exist this handler will be called.
async fn fallback() -> auth_response::Rejection {
    return auth_response::Rejection::InvalidRequest;
}

async fn auth_error(
    params: Query<AuthErrorParams>
) -> (StatusCode, Json<auth_response::ErrorMessage>) {
    (
        StatusCode::OK,
        auth_response::ErrorMessage::json(params.error.as_str(), "Description")
    )
}

#[derive(Deserialize)]
struct AuthErrorParams {
    error: String,
}

async fn authorization_grant(
    params: Query<AuthorizationGrantParams>
) -> auth_response::Result<Redirect> {
    // verify client_id, reject immediately and display error to user instead of redirect
    if !verify_client_id(&params.client_id) {
        return Err(auth_response::Rejection::InvalidClientId);
    }

    // validate redirect uri, inform the user of the problem instead of redirecting
    if !is_redirect_registered(&params.client_id, &params.redirect_uri) {
        return Err(auth_response::Rejection::InvalidRedirectUri);
    }

    if &params.response_type != "code" {
        return Err(auth_response::Rejection::UnsupportedResponseType(params.redirect_uri.clone()))
    }

    let parsed_scopes = params.scope.split(' ');
    let scopes = parsed_scopes.map(|s| s.to_string()).collect();
    
    if !verify_scopes(&params.client_id, &scopes) {
        return Err(auth_response::Rejection::InvalidScope(params.redirect_uri.clone()));
    }

    let code = models::Code::new(&params.client_id, scopes);
    let callback = format!("{}?code={}", &params.redirect_uri, &code.code);

    Ok(Redirect::temporary(callback.as_str()))
}

#[derive(Deserialize)]
struct AuthorizationGrantParams {
    pub response_type: String,
    pub client_id: String,
    pub redirect_uri: Url,
    pub scope: String,
    pub state: String,
    pub code_challenge: String,
    pub code_challange_method: String,
}

#[derive(Serialize)]
struct AuthorizationPayload {
    pub code: String,
    pub state: String,
}

async fn token(
    extractor::ExtractTokenFromGrant(token): extractor::ExtractTokenFromGrant,
) -> auth_response::Result<Json<models::Token>> {
    Ok(Json(token?))
}

