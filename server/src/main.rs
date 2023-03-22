// TODO
// [ ] scope handling
// [ ] better redirect/errors
// [x] auth code generation
// [x] device code
//     [x] generation
//     [x] user code generation
//     [x] verification uri
// [ ] auth code page
// [ ] middlewares
// [ ] description handling
// [ ] add PKCE support to auth code flow
// [ ] client registration

mod schema;
mod db;
mod auth_response;
mod models;
mod extract;

use std::net::SocketAddr;

use axum::{
    Router,
    body::{boxed, Body},
    extract::{Json, Query},
    http::{Response, StatusCode},
    response::Redirect,
    routing::{get, post},
};
use axum_macros::debug_handler;
use serde::Deserialize;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::ServeDir;
use url::Url;

/// TODO
/// Coordinate with database and assert that scopes exist and that the provided client_id has
/// access to use all of them 
/// For now just assume this is set up and return true
fn verify_scopes(client_id: &String, scopes: &String) -> bool {
    return true;
}

/// rfc: https://www.rfc-editor.org/rfc/rfc6749#section-4
#[tokio::main]
async fn main() {
    let api_routes = Router::new()
        .route("/authorize", post(handle_authorize))
        .route("/device/code", post(handle_device_auth_code))
        .route("/token", post(handle_token_request))
        .route("/error", get(handle_auth_error));

    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(get(|req| async move {
            match ServeDir::new(String::from("./dist")).oneshot(req).await {
                Ok(res) => res.map(boxed),
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {}", err))))
                    .expect("error response"),
            }
        }))
        .with_state(());

    // run it with hyper on localhost:8080
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
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

async fn handle_auth_error(
    params: Query<AuthErrorParams>
) -> (StatusCode, Json<auth_response::ErrorMessage>) {
    let error = auth_response::Rejection::from(params.error.as_str());
    let desc = error.into_error_description();

    (
        StatusCode::OK,
        auth_response::ErrorMessage::json(&params.error, desc)
    )
}

#[derive(Deserialize)]
struct AuthErrorParams {
    error: String,
}

async fn handle_authorize(
    extract::ExtractClientCredentials(client_credentials): extract::ExtractClientCredentials,
    params: Query<AuthorizeParams>
) -> auth_response::Result<Redirect> {
    if &params.response_type != "code" {
        return Err(auth_response::Rejection::UnsupportedResponseType(params.redirect_uri.clone()))
    }

    let client = client_credentials.validate()?;

    if !verify_scopes(&client.get_id(), &params.scope) {
        return Err(auth_response::Rejection::InvalidScope(params.redirect_uri.clone()));
    }

    // validate redirect uri, inform the user of the problem instead of redirecting
    models::RedirectUri::new(&client).validate(&params.redirect_uri)?;

    let scopes: Vec<String> = params.scope.split(' ').map(|s| s.to_string()).collect();
    let is_plain = !params.code_challenge_method.eq("S256");

    let code = models::AuthorizationCode::try_generate(
        &client,
        &params.code_challenge.as_str(), 
        is_plain, 
        &params.redirect_uri, 
        scopes
    )?;

    let callback = format!("{}?code={}&state={}", &params.redirect_uri, &code, &params.state);
    Ok(Redirect::temporary(callback.as_str()))
}

#[derive(Deserialize)]
struct AuthorizeParams {
    pub response_type: String,
    pub redirect_uri: Url,
    pub scope: String,
    pub state: String,
    pub code_challenge: String,
    pub code_challenge_method: String,
}

async fn handle_device_auth_code(
    extract::ExtractClientCredentials(client_credentials): extract::ExtractClientCredentials,
    Query(params): Query<DeviceAuthCodeParams>
) -> auth_response::Result<Json<models::DeviceCodeResponse>> {
    let client = client_credentials.validate()?;
    let scopes = params.scope.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();
    let device_code = models::DeviceCode::new(client, scopes);
    Ok(
        Json(device_code.try_generate_code()?)
    )
}

#[derive(Deserialize)]
struct DeviceAuthCodeParams {
    pub scope: String, 
}

#[derive(Debug, Deserialize)]
struct TokenRequest {
    grant_type: String,

    // authorization code
    redirect_uri: Option<Url>,
    code: Option<String>,
    code_verifier: Option<String>,

    // device authorization
    device_code: Option<String>,

    // refresh token 
    refresh_token: Option<String>,
}

#[debug_handler]
async fn handle_token_request(
    extract::ExtractClientCredentials(client_credentials): extract::ExtractClientCredentials,
    Query(params): Query<TokenRequest>
) -> auth_response::Result<Json<models::TokenResponse>> {
    match params.grant_type.as_str() {
        "authorization_code" => {
            return handle_authorization_code(client_credentials, params);
        },
        "client_credentials" => {
            return handle_client_credentials(client_credentials, params);
        },
        "device_code" => {
            return handle_device_code(client_credentials, params);
        },
        "refresh_token" => {
            return handle_refresh_token(client_credentials, params);
        }
        _ => Err(auth_response::Rejection::UnsupportedGrantType),
    }
}

fn handle_authorization_code(
    client_credentials: models::ClientCredentials,
    params: TokenRequest
) -> auth_response::Result<Json<models::TokenResponse>> {
    let Some(redirect_uri) = &params.redirect_uri
    else {
        return Err(auth_response::Rejection::InvalidRequest);
    };

    let Some(code) = &params.code
    else {
        return Err(auth_response::Rejection::InvalidRequest);
    };

    let Some(code_verifier) = &params.code_verifier
    else {
        return Err(auth_response::Rejection::InvalidRequest);
    };

    let client = client_credentials.validate()?;

    // validate authorization code
    // and get scopes
    models::AuthorizationCode::validate(&client, code, code_verifier.as_str(), &redirect_uri)?;

    // create token
    let token = models::TokenBuilder::new(
        client,
        String::from("todo"),
        Some(redirect_uri.clone())
    ).try_build()?;

    Ok(Json(models::TokenResponse::from(token)))
}

fn handle_client_credentials(
    client_credentials: models::ClientCredentials,
    params: TokenRequest,
) -> auth_response::Result<Json<models::TokenResponse>> {
    let client = client_credentials.validate()?;
    match client.get_type() {
        models::ClientType::Confidential => {();},
        models::ClientType::Public => return Err(auth_response::Rejection::InvalidClientId),
    }

    let token = models::TokenBuilder::new(
        client,
        String::from("todo"),
        None
    ).try_build()?;

    Ok(Json(models::TokenResponse::from(token)))
}

fn handle_device_code(
    client_credentials: models::ClientCredentials,
    params: TokenRequest
) -> auth_response::Result<Json<models::TokenResponse>> {
    let Some(device_code) = params.device_code
    else {
        return Err(auth_response::Rejection::InvalidRequest);
    };

    let client = client_credentials.validate()?;
    // validate device code
       

    let token = models::TokenBuilder::new(
        client,
        String::from("todo"),
        None
    ).try_build()?;

    Ok(Json(models::TokenResponse::from(token)))
}

fn handle_refresh_token(
    client_credentials: models::ClientCredentials,
    params: TokenRequest
) -> auth_response::Result<Json<models::TokenResponse>> {
    let Some(refresh_token) = params.refresh_token
    else {
        return Err(auth_response::Rejection::InvalidRequest);
    };

    let client = client_credentials.validate()?;
    // validate refresh token and mark as used 
    
    let token = models::TokenBuilder::new(
        client,
        String::from("todo"),
        None
    ).try_build()?;

    Ok(Json(models::TokenResponse::from(token)))
}

