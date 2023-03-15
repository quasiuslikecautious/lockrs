mod schema;
mod db;
mod auth_response;
mod models;
mod extract;

use std::net::SocketAddr;

use axum::{
    extract::{ Json, Query },
    http::StatusCode,
    response::Redirect,
    Router,
    routing::{ get, post, },
};
use axum_macros::debug_handler;
use serde::{ Deserialize, Serialize };
use url::Url;
use uuid::Uuid;

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
fn verify_scopes(client_id: &String, scopes: &String) -> bool {
    return true;
}

/// rfc: https://www.rfc-editor.org/rfc/rfc6749#section-4
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/authorize", post(handle_authorize))
        .route("/device/code", get(handle_device_auth_code))
        .route("/token", post(handle_token_request))
        .route("/error", get(handle_auth_error))
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

async fn handle_auth_error(
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

async fn handle_authorize(
    params: Query<AuthorizeParams>
) -> auth_response::Result<Redirect> {
    // validate redirect uri, inform the user of the problem instead of redirecting
    if !is_redirect_registered(&params.client_id, &params.redirect_uri) {
        return Err(auth_response::Rejection::InvalidRedirectUri);
    }

    if &params.response_type != "code" {
        return Err(auth_response::Rejection::UnsupportedResponseType(params.redirect_uri.clone()))
    }

    if !verify_scopes(&params.client_id, &params.scope) {
        return Err(auth_response::Rejection::InvalidScope(params.redirect_uri.clone()));
    }

    let code = models::Code::new(&params.client_id, params.scope.clone());
    let callback = format!("{}?code={}&state={}", &params.redirect_uri, &code.code, &params.state);

    Ok(Redirect::temporary(callback.as_str()))
}

#[derive(Deserialize)]
struct AuthorizeParams {
    pub response_type: String,
    pub client_id: String,
    pub redirect_uri: Url,
    pub scope: String,
    pub state: String,
    pub code_challenge: String,
    pub code_challange_method: String,
}

#[derive(Serialize)]
struct AuthorizePayload {
    pub code: String,
    pub state: String,
}

async fn handle_device_auth_code() {
    todo!("issue device code");
}

#[derive(Debug, Deserialize)]
struct TokenRequest {
    grant_type: String,

    // client id for public clients
    user_id: Option<Uuid>,
    client_id: Option<Uuid>,
    
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
    extract::ExtractClientCredentials(client): extract::ExtractClientCredentials,
    Query(params): Query<TokenRequest>
) -> auth_response::Result<Json<models::TokenPayload>> {
    match params.grant_type.as_str() {
        "authorization_code" => {
            return handle_authorization_code(client, params);
        },
        "client_credentials" => {
            return handle_client_credentials(client, params);
        },
        "device_code" => {
            return handle_device_code(client, params);
        },
        "refresh_token" => {
            return handle_refresh_token(client, params);
        }
        _ => Err(auth_response::Rejection::UnsupportedGrantType),
    }
}

fn handle_authorization_code(
    client: models::ValidatedClient,
    params: TokenRequest
) -> auth_response::Result<Json<models::TokenPayload>> {
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

    let Some(user_id) = &params.user_id
    else {
        return Err(auth_response::Rejection::InvalidRequest);
    };

    let unvalidated_user = models::UnvalidatedUser::new(*user_id);
    let validated_user = unvalidated_user.validate(&redirect_uri)?;

    // validate authorization code
    // and get scopes

    // create token
    let token = models::TokenBuilder::new(
        client,
        Some(validated_user),
        String::from("todo"),
        Some(redirect_uri.clone())
    ).try_build()?;

    Ok(Json(models::TokenPayload::from(token)))
}

fn handle_client_credentials(
    client: models::ValidatedClient,
    params: TokenRequest,
) -> auth_response::Result<Json<models::TokenPayload>> {
    match client.get_type() {
        models::ClientType::Confidential => {();},
        models::ClientType::Public => return Err(auth_response::Rejection::InvalidClientId),
    }

    let token = models::TokenBuilder::new(
        client,
        None,
        String::from("todo"),
        None
    ).try_build()?;

    Ok(Json(models::TokenPayload::from(token)))
}

fn handle_device_code(
    client: models::ValidatedClient,
    params: TokenRequest
) -> auth_response::Result<Json<models::TokenPayload>> {
    let Some(device_code) = params.device_code
    else {
        return Err(auth_response::Rejection::InvalidRequest);
    };

    // validate device code
    

    let token = models::TokenBuilder::new(
        client,
        None,
        String::from("todo"),
        None
    ).try_build()?;

    Ok(Json(models::TokenPayload::from(token)))
}

fn handle_refresh_token(
    client: models::ValidatedClient,
    params: TokenRequest
) -> auth_response::Result<Json<models::TokenPayload>> {
    let Some(refresh_token) = params.refresh_token
    else {
        return Err(auth_response::Rejection::InvalidRequest);
    };

    // validate refresh token and mark as used 
    
    let token = models::TokenBuilder::new(
        client,
        None, // TODO get user_id if exists when validating refresh_token
        String::from("todo"),
        None,
    ).try_build()?;

    Ok(Json(models::TokenPayload::from(token)))
}

