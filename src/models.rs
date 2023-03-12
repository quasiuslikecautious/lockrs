use serde::{ Deserialize, Serialize, };
use url::Url;

/// Use the HTTP Basic authentication scheme to authenticate with the authentication server. The
/// client identifier is encoded using the "application/x-www-form-urlencoded" encoding algorthm,
/// and the encoded value is used as the username; the client password is encoded using the same
/// algorithm and used as the password.
#[derive(Debug, Serialize)]
pub struct Client {
    client_id: String,
    client_secret: Option<String>,

    #[serde(skip_serializing)]
    client_type: ClientType,
}

impl Client {
    pub fn new(
        client_id: &str, 
        client_secret: Option<String>
    ) -> Self {
        let client_type = match &client_secret {
            Some(_) => ClientType::Public,
            None => ClientType::Confidential,
        };

        Self {
            client_id: client_id.to_string(),
            client_secret,
            client_type,
        }
    }
}

#[derive(Deserialize)]
pub struct ClientIdQueryParam {
    pub client_id: String,
}

#[derive(Deserialize)]
pub struct GrantTypeQueryParam {
    pub grant_type: String,
}

/// The client type designation is based on the authorization server's definition of secure
/// authentication and its acceptable exposure levels of client credentials. The authorization
/// server SHOULD NOT make assumptions about the client type.
#[derive(Debug)]
pub enum ClientType {
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

/// The authorization grant code supplied in the authorization grant step of the auth flow
#[derive(Serialize)]
pub struct Code {
    pub code: String,
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
pub struct Token {
    pub token_type: String, // usually just 'Bearer'
    pub expires_in: i64,
    pub access_token: String, // 10 minutes
    pub refresh_token: Option<String>, // 24 hours
    pub scopes: Vec<String>,
}

impl Token {
    pub fn new(
        token_type: String,
        expires_in: i64,
        access_token: String,
        refresh_token: Option<String>,
        scopes: Vec<String>
    ) -> Self {
        Self {
            token_type,
            expires_in,
            access_token,
            refresh_token,
            scopes,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthorizationCodeParams {
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: Url, // url to enforce absolute url instead of relative path
    pub code_verifier: String, // require pkce for best practices/draft of oauth 2.1
    pub client_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ClientCredentialsParams {
    pub grant_type: String,
    pub scope: String,
    pub client_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeviceCodeParams {
    pub grant_type: String,
    pub device_code: String,
    pub client_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenParams {
    pub grant_type: String,
    pub refresh_token: String,
    pub scope: String,
    pub client_id: Option<String>,
}

