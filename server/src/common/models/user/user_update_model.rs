use diesel::prelude::*;

use crate::pg::schema::users;

#[derive(Debug, AsChangeset)]
#[diesel(primary_key(id), table_name = users)]
pub struct UserUpdateModel {
    pub email: Option<String>,
}
