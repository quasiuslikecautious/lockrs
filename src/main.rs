use std::net::SocketAddr;

use axum::{
    extract::{ Json, Query },
    http::Uri,
    Router,
    routing::{ get, post, },
};
use axum_macros::debug_handler;
use serde::{ Deserialize, Serialize };


#[derive(Serialize)]
struct Code {
    code: String,
}

impl Code {
    pub fn new<S: Into<String>>(code: S)-> Self {
        Self {
            code: code.into(),
        }
    }
}

#[derive(Serialize)]
struct Token {
    token_type: String,
    expires_in: i64,
    access_token: String,
    refresh_token: String,
}

pub mod result {
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
    };

    pub enum Error {
        BadRoute,
        BadClientId,
        BadRedirectUri,
    }

    impl IntoResponse for Error {
        fn into_response(self) -> Response {
            let (status_code, body) = match self {
                Error::BadRoute         => (StatusCode::NOT_FOUND, "Not found"),
                Error::BadClientId      => (StatusCode::UNAUTHORIZED, "Invalid client id supplied"),
                Error::BadRedirectUri   => (StatusCode::BAD_REQUEST, "Invalid redirect uri supplied"),
            };

            (status_code, body).into_response()
        }
    }

    pub type Result<T> = std::result::Result<T, Error>;
}

fn verify_client_id(client_id: &String) -> bool {
    return true;
}

fn is_redirect_registered(client_id: &String, redirect_uri: &Uri) -> bool {
    return true;
}

/// rfc: https://www.rfc-editor.org/rfc/rfc6749#section-4
#[tokio::main]
async fn main() {
    // initiates the authorization flow and returns the authorization code to the clien
    let authorize_routes = Router::new()
        .route("/auth", post(authorization_grant));

    let app = Router::new()
        .merge(authorize_routes)
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

async fn fallback() -> result::Error {
    return result::Error::BadRoute;
}

async fn authorization_grant(
    params: Query<AuthorizationGrantParams>
) -> result::Result<Json<Code>> {
    // verify client_id, reject immediately and display error to user instead of redirect
    if !verify_client_id(&params.client_id) {
        return Err(result::Error::BadClientId);
    }

    // validate redirect uri, inform the user of the problem instead of redirecting
    if !is_redirect_registered(&params.client_id, &params.redirect_uri) {
        return Err(result::Error::BadRedirectUri);
    }

    let code: Code = Code::new("Hello, OAuth!");

    Ok(Json(code))
}

#[derive(Deserialize)]
struct AuthorizationGrantParams {
    pub response_type: String,
    pub client_id: String,
    #[serde(with = "http_serde::uri")]
    pub redirect_uri: Uri,
    pub scope: String,
    pub state: String,
}

/// Authorization Code Grant
/// https://oauth.net/2/grant-types/authorization-code/
/// 
/// The Authorization Code grant type is used by confidential and public clients to exchange an 
/// authorization code for an access token. After the user returns to the client via the redirect
/// URL, the application will get the authorization code from the URL and use it to request an
/// access token. It is recommended that all clients use the PKCE extension with this flow as well 
/// to provide better security.
async fn authorization_code(
    params: Query<AuthorizationCodeParams>
) -> result::Result<Json<Token>> {
    todo!();
}

#[derive(Deserialize)]
struct AuthorizationCodeParams {
    pub grant_type: String,
    pub code: String,
    #[serde(with = "http_serde::uri")]
    pub redirect_uri: Uri,
    pub client_id: String,
}

/// Proof Key for Code Exchange
/// https://oauth.net/2/pkce/
/// 
/// PKCE (RFC 7636) is an extension to the Authorization Code flow to prevent CSRF and
/// authorization code injection attacks. PKCE is not a replacement for a client secret, and PKCE
/// is recommended even if a client is using a client secret.
/// 
/// Note: Because PKCE is not a replacement for client authentication, it does not allow treating a
/// public client as a confidential client. PKCE was originally designed to protect the 
/// authorization code flow in mobile apps, but its ability to prevent authorization code injection
/// makes it useful for every type of OAuth client, even web apps that use a client secret.
async fn pkce(
    params: Query<PkceParams>
) -> result::Result<Json<Token>> {
    todo!();
}

#[derive(Deserialize)]
struct PkceParams {
    pub grant_type: String,
    pub code: String,
    #[serde(with = "http_serde::uri")]
    pub redirect_uri: Uri,
    pub code_verifier: String,
}

/// Client Credentials
/// https://oauth.net/2/grant-types/client-credentials/
/// 
/// The Client Credentials grant type is used by clients to obtain an access token outside of the
/// context of a user. This is typically used by clients to access resources about themselves
/// rather than to access a user's resources.
async fn client_credentials(
    params: ClientCredentialsParams
) -> result::Result<Json<Token>> {
    todo!();
}

#[derive(Deserialize)]
struct ClientCredentialsParams {
    pub grant_type: String,
    pub scope: String,
    pub client_id: String,
    pub client_secret: String,
}

/// Device Authorization
/// https://oauth.net/2/grant-types/device-code/
/// 
/// The Device Code grant type is used by browserless or input-constrained devices in the device
/// flow to exchange a previously obtained device code for an access token. The Device Code grant
/// type value is urn:ietf:params:oauth:grant-type:device_code.
async fn device_code(

) -> result::Result<Json<Token>> {
    todo!();
}

/// Access Token
/// https://oauth.net/2/access-tokens/
/// 
/// An OAuth Access Token is a string that the OAuth client uses to make requests to the resource
/// server. Access tokens do not have to be in any particular format, and in practice, various
/// OAuth servers have chosen many different formats for their access tokens. Access tokens may be
/// either "bearer tokens" or "sender-constrained" tokens. Sender-constrained tokens require the
/// OAuth client to prove possession of a private key in some way in order to use the access token,
/// such that the access token by itself would not be usable.
async fn access_token(

) -> result::Result<Json<Token>> {
    todo!();
}

/// Refresh Token
/// https://oauth.net/2/grant-types/refresh-token/
/// 
/// The Refresh Token grant type is used by clients to exchange a refresh token for an access token
/// when the access token has expired. This allows clients to continue to have a valid access token
/// without further interaction with the user.
async fn refresh_token(
    params: Query<RefreshTokenParams>
) -> result::Result<Json<Token>> {
    todo!();
}

#[derive(Deserialize)]
struct RefreshTokenParams {
    pub grant_type: String,
    pub refresh_token: String,
    pub client_id: String,
    pub client_secret: String,
    pub scope: String,
}