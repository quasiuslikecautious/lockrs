use diesel::prelude::*;
use uuid::Uuid;

use crate::db::pg::schema::users;

#[derive(Debug, PartialEq)]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug)]
pub struct UserCreateModel {
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(primary_key(id), table_name = users)]
pub struct UserUpdateModel {
    pub email: Option<String>,
}
