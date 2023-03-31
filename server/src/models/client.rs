use base64::{Engine as _, engine::general_purpose};
use ring::rand::{SecureRandom, SystemRandom};
use url::Url;

use crate::{
    auth_response,
    db::{DbError, models::DbClient},
    models::User,
};

/// Use the HTTP Basic authentication scheme to authenticate with the authentication server. The
/// client identifier is encoded using the "application/x-www-form-urlencoded" encoding algorthm,
/// and the encoded value is used as the username; the client password is encoded using the same
/// algorithm and used as the password.
#[derive(Debug)]
pub struct ClientCredentials {
    id: String,
    secret: Option<String>,
    client_type: ClientType,
}

impl ClientCredentials {
    pub fn new(
        id: &String, 
        secret: Option<String>
    ) -> Self {
        let client_type = match &secret {
            Some(_) => ClientType::Public,
            None => ClientType::Confidential,
        };

        Self {
            id: id.clone(),
            secret,
            client_type,
        }
    }

    fn generate_random_string() -> String {
        let mut buffer = [0u8; 32];
        let rng = SystemRandom::new();
        rng.fill(&mut buffer).unwrap();
        general_purpose::URL_SAFE_NO_PAD.encode(buffer).to_string()
    }

    pub fn create(
        user: User,
        is_confidential: bool,
        name: String,
        description: String,
        homepage_url: Url,
        redirect_url: Url
    ) -> auth_response::Result<Client> {
        let client_id = Self::generate_random_string();

        let client_secret = match is_confidential {
            true => Some(Self::generate_random_string()),
            false => None,
        };

        let db_client = DbClient::insert(
            &client_id,
            &client_secret,
            &user.get_id(),
            &name,
            &description,
            &homepage_url,
            &redirect_url
        ).map_err(|err| {
            match err {
                _ => auth_response::Rejection::ServerError(None),
            }
        })?;

        Ok(Client::from(db_client))
    }

    pub fn validate(&self) -> auth_response::Result<Client> {
        DbClient::get(&self.id, &self.secret)
            .map_err(|err| {
                match err {
                    DbError::NotFound       => auth_response::Rejection::InvalidClientId,
                    DbError::InternalError  => auth_response::Rejection::ServerError(None),
                }
            })?;

        Ok(Client {
            id: self.id.clone(),
            secret: self.secret.clone(),
            client_type: self.client_type,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    id: String,
    secret: Option<String>,
    client_type: ClientType,
}

impl Client {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_secret(&self) -> Option<String> {
        self.secret.clone()
    }

    pub fn get_type(&self) -> ClientType {
        self.client_type
    }
}

impl From<DbClient> for Client {
    fn from(db_client: DbClient) -> Self {
        let client_type = match db_client.secret {
                Some(_) => ClientType::Confidential,
                None => ClientType::Public,
        };

        Self {
            id: db_client.id,
            secret: db_client.secret,
            client_type,
        }
    }
}

/// The client type designation is based on the authorization server's definition of secure
/// authentication and its acceptable exposure levels of client credentials. The authorization
/// server SHOULD NOT make assumptions about the client type.
#[derive(Copy, Clone, Debug)]
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

