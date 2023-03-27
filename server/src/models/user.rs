use diesel::prelude::*;
use url::Url;
use uuid::Uuid;

use crate::{
    auth_response::{Result, Rejection},
    db,
};


#[derive(Debug)]
pub struct UserCredentials {
    id: Uuid,
}

impl UserCredentials {
    pub fn new(id: &Uuid) -> Self {
        Self {
            id: id.clone(),
        }
    }

    pub fn validate(&self, redirect_uri: &Url) -> Result<User> {
        use crate::schema::users::dsl::*;
        let connection = &mut db::establish_connection();
        let transaction = connection.build_transaction()
            .read_only()
            .run(|conn| {
                users
                    .filter(id.eq(&self.id))
                    .first::<db::DbUser>(conn)
            })
            .map_err(|_| Rejection::AccessDenied(redirect_uri.clone()))?;

        Ok(User{
            id: self.id,
        })
    }
}

#[derive(Clone, Debug)]
pub struct User {
    id: Uuid,
}

impl User {
    pub fn get_id(&self) -> Uuid {
        self.id
    }
}

