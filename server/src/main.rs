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

mod db;
mod api_response;
mod auth_response;
mod codes;
mod tokens;
mod models;
mod extract;

use std::net::SocketAddr;

use axum::{
    Router,
    body::{boxed, Body},
    extract::{Json, Path, Query},
    http::{Response, StatusCode},
    response::Redirect,
    routing::{get, post, put},
};
use auth_response::Rejection;
use axum_macros::debug_handler;
use serde::Deserialize;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};
use url::Url;
use uuid::{uuid, Uuid};

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

    let oauth_routes = Router::new()
        .route("/authorize", post(handle_authorize))
        .route("/device/code", post(handle_device_auth_code))
        .route("/token", post(handle_token_request))
        .route("/error", get(handle_auth_error));

    let auth_routes = Router::new()
        .route("/user/create", put(handle_create_user))
        .route("/user/login", post(handle_authenticate_user))
        .route("/user/:user_id", get(handle_get_user))
        .route("/client/create", put(handle_create_client))
        .route("/client/:client_id", get(handle_get_client));

    let api_routes = Router::new()
        .nest("/oauth", oauth_routes)
        .nest("/auth", auth_routes);

    let app = Router::new()
        .nest("/api/v1", api_routes)
        .fallback_service(get(|req| async move {
            match ServeDir::new(String::from("./dist")).oneshot(req).await {
                Ok(res) => res.map(boxed),
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {}", err))))
                    .expect("error response"),
            }
        }))
        .layer(TraceLayer::new_for_http())
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
async fn fallback() -> Rejection {
    return Rejection::InvalidRequest;
}


async fn handle_create_user(
    extract::BasicAuth((email, password)): extract::BasicAuth
) -> StatusCode {
    let new_user = models::UserCredentials::create_from_credentials(
        email, 
        password
    );

    match new_user {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

async fn handle_authenticate_user(
    extract::BasicAuth((email, password)): extract::BasicAuth
) -> StatusCode {
    match models::UserCredentials::get_from_credentials(email, password) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::UNAUTHORIZED,
    }
}

async fn handle_get_user(Path(user_id): Path<Uuid>) {
    
}

async fn handle_create_client(
    extract::BasicAuth((email, password)): extract::BasicAuth,
    Json(request_body): Json<ClientCreateRequest>
) -> StatusCode {
    let Some(user) = models::UserCredentials::get_from_credentials(email, password).ok()
    else {
        return StatusCode::UNAUTHORIZED;
    };

    let client_create_attempt = models::ClientCredentials::create(
        user,
        request_body.is_confidential,
        request_body.name,
        request_body.description,
        request_body.homepage_url,
        request_body.callback_url
    );

    match client_create_attempt {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

#[derive(Deserialize)]
struct ClientCreateRequest {
    pub name: String,
    pub is_confidential: bool,
    pub homepage_url: Url,
    pub description: String,
    pub callback_url: Url,
}

async fn handle_get_client(Path(client_id): Path<String>) {
    
}

async fn handle_auth_error(
    params: Query<AuthErrorParams>
) -> (StatusCode, Json<auth_response::ErrorMessage>) {
    let error = Rejection::from(params.error.as_str());
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
        return Err(Rejection::UnsupportedResponseType(params.redirect_uri.clone()))
    }
    
    let client = client_credentials.validate()?;

    // validate redirect uri, inform the user of the problem instead of redirecting
    models::RedirectUri::validate(&client, &params.redirect_uri)?;

    let Some(scopes) = models::ScopeRequest::get_validated_scopes(params.scope.as_str())
    else {
        return Err(Rejection::InvalidScope(Some(params.redirect_uri.clone())));  
    };

    let is_plain = !params.code_challenge_method.eq("S256");

    let code = codes::AuthorizationCode::try_generate(
        &client,
        &params.code_challenge, 
        is_plain,
        &params.redirect_uri, 
        scopes.get().to_owned()
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

    let Some(scopes) = models::ScopeRequest::get_validated_scopes(params.scopes.as_str())
    else {
        return Err(Rejection::InvalidScope(None));  
    };

    let device_code = codes::DeviceCode::try_generate_code(client, scopes.get().to_owned())?;

    Ok(
        Json(device_code)
    )
}

#[derive(Deserialize)]
struct DeviceAuthCodeParams {
    pub scopes: String, 
}

#[derive(Debug, Deserialize)]
struct TokenRequest {
    grant_type: String,

    scopes: Option<String>,
    user_id: Option<Uuid>,

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
        _ => Err(Rejection::UnsupportedGrantType),
    }
}

fn handle_authorization_code(
    client_credentials: models::ClientCredentials,
    params: TokenRequest
) -> auth_response::Result<Json<models::TokenResponse>> {
    let Some(redirect_uri) = &params.redirect_uri
    else {
        return Err(Rejection::InvalidRequest);
    };

    let Some(user_id) = &params.user_id
    else {
        return Err(Rejection::InvalidRequest);
    };

    let Some(code) = &params.code
    else {
        return Err(Rejection::InvalidRequest);
    };

    let Some(code_verifier) = &params.code_verifier
    else {
        return Err(Rejection::InvalidRequest);
    };

    let client = client_credentials.validate()?;
    let user = models::UserCredentials::new(user_id)
        .validate(redirect_uri)?;

    // validate authorization code
    // and get scopes
    let scopes = codes::AuthorizationCode::validate(&client, code, &code_verifier, &redirect_uri)?;

    // create token
    let token = tokens::TokenBuilder::new(
        client,
        Some(user),
        scopes,
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
        models::ClientType::Public => return Err(Rejection::InvalidClientId),
    }

    let Some(scopes) = &params.scopes
    else {
        return Err(Rejection::InvalidRequest);
    };

    let Some(scopes) = models::ScopeRequest::get_validated_scopes(scopes)
    else {
        return Err(Rejection::InvalidScope(None));
    };

    let token = tokens::TokenBuilder::new(
        client,
        None,
        scopes,
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
        return Err(Rejection::InvalidRequest);
    };

    let client = client_credentials.validate()?;
    // validate device code

    let token = tokens::TokenBuilder::new(
        client,
        None, // TODO => Get user_id from device_code after authentication
        models::Scopes::new(vec![]), // TODO
        None
    ).try_build()?;

    Ok(Json(models::TokenResponse::from(token)))
}

fn handle_refresh_token(
    client_credentials: models::ClientCredentials,
    params: TokenRequest
) -> auth_response::Result<Json<models::TokenResponse>> {
    let Some(refresh_token) = &params.refresh_token
    else {
        return Err(Rejection::InvalidRequest);
    };

    let client = client_credentials.validate()?;
    // validate refresh token and mark as used 
    
    let user_id = uuid!("00000000-0000-0000-0000-000000000000");
    let user = models::UserCredentials::new(&user_id).validate(&Url::parse("http://::1").unwrap());
    
    let scopes = tokens::TokenBuilder::validate_refresh_token(&client, &None, &refresh_token)?;

    let token = tokens::TokenBuilder::new(
        client,
        None, // TODO Get user from refresh_token
        scopes,
        None
    ).try_build()?;

    Ok(Json(models::TokenResponse::from(token)))
}

