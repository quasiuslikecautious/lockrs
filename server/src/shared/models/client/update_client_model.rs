use diesel::prelude::*;
use serde::Deserialize;

use crate::db::schema::clients;

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = clients)]
pub struct UpdateClientModel {
    pub name: Option<String>,
    pub description: Option<String>,
    pub homepage_url: Option<String>,
}

