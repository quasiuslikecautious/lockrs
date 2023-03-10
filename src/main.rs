use std::net::SocketAddr;

use axum::{
    extract::{ Json, Query },
    response::Redirect,
    Router,
    routing::{ get, post, },
};
use axum_macros::debug_handler;
use serde::{ Deserialize, Serialize };
use url::Url;

/// The client type designation is based on the authorization server's definition of secure
/// authentication and its acceptable exposure levels of client credentials. The authorization
/// server SHOULD NOT make assumptions about the client type.
enum ClientType {
    /// Clients capable of maintaining the confidentiality of their credentials (e.g., client
    /// implements on a secure server with restricted access to the client credentials), or capable
    /// of secure client authentication using other means
    Confidential,

    /// Clients incapable of maintaining the confidentiality of their credentials (e.g. clients
    /// executing on the device used by a resource owner, such as an installed native application
    /// or a web browser-based application), and incapable of secure client authentication via any
    /// other means
    Public,
}

impl ClientType {
    pub fn get_value(self) -> String {
        match self {
            ClientType::Confidential    => "confidential".to_string(),
            ClientType::Public          => "public".to_string(),
        }
    }
}

/// Use the HTTP Basic authentication scheme to authenticate with the authentication server. The
/// client identifier is encoded using the "application/x-www-form-urlencoded" encoding algorthm,
/// and the encoded value is used as the username; the client password is encoded using the same
/// algorithm and used as the password.
#[derive(Serialize)]
struct Client {
    client_id: String,
    client_secret: String,
}

/// The authorization grant code supplied in the authorization grant step of the auth flow
#[derive(Serialize)]
struct Code {
    code: String,
}

impl Code {
    pub fn new(client_id: &String, requested_scopes: Vec<String>)-> Self {
        Self {
            code: Self::generate(&client_id, requested_scopes),
        }
    }

    /// TODO
    /// Connect to database, and generate some code based off of params provided
    pub fn generate(client_id: &String, requested_scopes: Vec<String>) -> String {
        let code = format!("{}:{:?}", &client_id, &requested_scopes);
        code
    }

    /// TODO
    /// Decrypt/Verify (and remove from db if necessary) provided code
    pub fn verify(code: String) -> bool {
        return true;
    }
}

/// The auth token provided when a client has successfully authorized through the grant flow
#[derive(Serialize)]
struct Token {
    token_type: String,
    expires_in: i64,
    access_token: String,
    refresh_token: String,
}

mod result {
    use axum::{
        extract::Json,
        response::{IntoResponse, Response, Redirect},
    };
    use serde::Serialize;
    use url::Url;

    #[derive(Serialize)]
    pub struct ErrorMessage {
        pub error: String,
        pub error_desciption: String,
    }

    impl ErrorMessage {
        pub fn new(error: &str, error_description: &str) -> Self {
            Self {
                error: error.to_string(),
                error_desciption: error_description.to_string(),
            }
        }

        pub fn json(error: &str, error_description: &str) -> Json<Self> {
            Json(Self::new(error, error_description))
        }
    }

    pub enum Error {
        InvalidRequest,
        AccessDenied(Url),
        ServerError(Url),
        TemporarilyUnavailable(Url),

        InvalidClientId,
        InvalidRedirectUri,
        UnsupportedResponseType(Url),
        InvalidScope(Url),
    }

    impl Error {
        pub fn into_callback_url(&self) -> String {
            let default_callback_url = String::from("http://127.0.0.1:8080");

            match self {
                Self::InvalidRequest => default_callback_url.to_string(),
                Self::AccessDenied(callback) => callback.to_string(),
                Self::ServerError(callback) => callback.to_string(),
                Self::TemporarilyUnavailable(callback) => callback.to_string(),
                Self::InvalidClientId => default_callback_url,
                Self::InvalidRedirectUri => default_callback_url,
                Self::UnsupportedResponseType(callback) => callback.to_string(),
                Self::InvalidScope(callback) => callback.to_string(),
            }
        }

        pub fn into_error_code(&self) -> &'static str {
            match self {
                Self::InvalidRequest => "invalid_request",
                Self::AccessDenied(_) => "access_denied",
                Self::ServerError(_) => "server_error",
                Self::TemporarilyUnavailable(_) => "temporary_error",
                Self::InvalidClientId => "invalid_client",
                Self::InvalidRedirectUri => "invalid_redirect",
                Self::UnsupportedResponseType(_) => "unsupported_response_type",
                Self::InvalidScope(_) => "invalid_scope",
            }
        }

        pub fn into_error_description(&self) -> &'static str {
            match self {
                Self::InvalidRequest => "Invalid Request",
                Self::AccessDenied(_) => "The resource owner has denied the authorization request",
                Self::ServerError(_) => "An internal error occured while processing your request",
                Self::TemporarilyUnavailable(_) => "Please try again later",
                Self::InvalidClientId => "Invalid client_id supplied",
                Self::InvalidRedirectUri => "Invalid redirect_uri supplied",
                Self::UnsupportedResponseType(_) => "Unsupported response_type requested",
                Self::InvalidScope(_) => "Invalid scope(s) requested",
            }
        }
    }

    impl IntoResponse for Error {
        fn into_response(self) -> Response {
            let callback = self.into_callback_url();
            let error_code = self.into_error_code();

            let redirect_uri = format!("{}?error={}", &callback, &error_code);

            Redirect::to(redirect_uri.as_str()).into_response()
        }
    }

    pub type Result<T> = std::result::Result<T, Error>;
}

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
    // initiates the authorization flow and returns the authorization code to the clien
    let routes = Router::new()
        .route("/authorize", post(authorization_grant))
        .route("/token", post(access_token))
        .route("/refresh", post(refresh_token));

    let app = Router::new()
        .route("/error", get(auth_error))
        .merge(routes)
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
async fn fallback() -> result::Error {
    return result::Error::InvalidRequest;
}

async fn auth_error(
    params: Query<AuthErrorParams>
) -> Json<result::ErrorMessage> {
    result::ErrorMessage::json(params.error.as_str(), "Description")
}

#[derive(Deserialize)]
struct AuthErrorParams {
    error: String,
}

async fn authorization_grant(
    params: Query<AuthorizationGrantParams>
) -> result::Result<Redirect> {
    // verify client_id, reject immediately and display error to user instead of redirect
    if !verify_client_id(&params.client_id) {
        return Err(result::Error::InvalidClientId);
    }

    // validate redirect uri, inform the user of the problem instead of redirecting
    if !is_redirect_registered(&params.client_id, &params.redirect_uri) {
        return Err(result::Error::InvalidRedirectUri);
    }

    if &params.response_type != "code" {
        return Err(result::Error::UnsupportedResponseType(params.redirect_uri.clone()))
    }

    let parsed_scopes = params.scope.split(' ');
    let scopes = parsed_scopes.map(|s| s.to_string()).collect();
    
    if !verify_scopes(&params.client_id, &scopes) {
        return Err(result::Error::InvalidScope(params.redirect_uri.clone()));
    }

    let code: Code = Code::new(&params.client_id, scopes);
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
    pub redirect_uri: Url,
    pub code_verifier: String,
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
    pub redirect_uri: Url,
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
