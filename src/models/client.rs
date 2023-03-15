use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    auth_response,
    db,
};

/// Use the HTTP Basic authentication scheme to authenticate with the authentication server. The
/// client identifier is encoded using the "application/x-www-form-urlencoded" encoding algorthm,
/// and the encoded value is used as the username; the client password is encoded using the same
/// algorithm and used as the password.
#[derive(Debug)]
pub struct UnvalidatedClient {
    id: Uuid,
    secret: Option<String>,
    client_type: ClientType,
}

impl UnvalidatedClient {
    pub fn new(
        id: &Uuid, 
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

    pub fn validate(&self) -> auth_response::Result<ValidatedClient> {
        use crate::schema::clients::dsl::*;

        let mut query = clients
            .into_boxed()
            .filter(id.eq(&self.id));

        if let Some(client_secret) = &self.secret {
            query = query.filter(secret.eq(client_secret));
        }

        let connection = &mut db::establish_connection();
        connection.build_transaction()
            .read_only()
            .run(|conn| {
                query.first::<db::DbClient>(conn)
            })
            .map_err(|_| auth_response::Rejection::InvalidClientId)?;

        Ok(ValidatedClient {
            id: self.id,
            secret: self.secret.clone(),
            client_type: self.client_type,
        })
    }
}

#[derive(Debug)]
pub struct ValidatedClient {
    id: Uuid,
    secret: Option<String>,
    client_type: ClientType,
}

impl ValidatedClient {
    pub fn get_id(&self) -> Uuid {
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

