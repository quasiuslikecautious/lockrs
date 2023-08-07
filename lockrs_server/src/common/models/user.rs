use diesel::prelude::*;
use uuid::Uuid;

use crate::db::pg::schema::users;

#[derive(Debug, PartialEq)]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
}

impl UserModel {
    pub fn new(id: Uuid, email: &str) -> Self {
        Self {
            id,
            email: email.to_owned(),
        }
    }
}

#[derive(Debug, AsChangeset)]
#[diesel(primary_key(id), table_name = users)]
pub struct UserUpdateModel {
    pub email: Option<String>,
}

impl UserUpdateModel {
    pub fn new(email: Option<&str>) -> Self {
        Self {
            email: email.map(|s| s.to_owned()),
        }
    }
}
