use diesel::prelude::*;

use crate::pg::schema::scopes;

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = scopes)]
pub struct PgScope {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub client_id: Option<String>,
}
