use diesel::prelude::*;

use crate::pg::schema::clients;

#[derive(AsChangeset)]
#[diesel(table_name = clients)]
pub struct ClientUpdateModel {
    pub name: Option<String>,
    pub description: Option<String>,
    pub homepage_url: Option<String>,
}
