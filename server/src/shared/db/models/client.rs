use diesel::prelude::*;
use uuid::Uuid;

use crate::db::schema::clients;

#[derive(Debug, Queryable, Insertable, Identifiable)]
#[diesel(primary_key(id), table_name = clients)]
pub struct DbClient {
    pub id: String,
    pub secret: Option<String>,
    pub user_id: Uuid,
    pub is_public: bool,
    pub name: String,
    pub description: String,
    pub homepage_url: String,
}
