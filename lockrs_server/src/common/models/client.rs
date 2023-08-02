use diesel::prelude::*;
use uuid::Uuid;

use crate::db::pg::schema::clients;

#[derive(Debug, PartialEq)]
pub struct ClientModel {
    pub user_id: Uuid,
    pub id: String,
    pub is_public: bool,
    pub name: String,
    pub description: String,
    pub homepage_url: String,
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = clients)]
pub struct ClientUpdateModel {
    pub name: Option<String>,
    pub description: Option<String>,
    pub homepage_url: Option<String>,
}
