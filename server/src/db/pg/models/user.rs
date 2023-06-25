use diesel::prelude::*;
use uuid::Uuid;

use crate::pg::schema::users;

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = users)]
pub struct PgUser {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}
