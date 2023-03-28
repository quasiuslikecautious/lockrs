use crate::{
    auth_response,
    db::{DbError, models::DbClient},
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

