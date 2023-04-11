use diesel::prelude::*;
use uuid::Uuid;

use crate::db::schema::users;

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = users)]
pub struct DbUser {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

