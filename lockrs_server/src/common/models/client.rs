use diesel::prelude::*;
use url::Url;
use uuid::Uuid;

use crate::db::pg::schema::clients;

#[derive(Debug, PartialEq)]
pub struct ClientModel {
    pub user_id: Uuid,
    pub id: String,
    pub secret: Option<String>,
    pub name: String,
    pub description: String,
    pub homepage_url: String,
}

#[derive(Debug)]
pub struct ClientAuthModel {
    pub id: String,
    pub secret: Option<String>,
}

#[derive(Debug)]
pub struct ClientCreateModel {
    pub user_id: Uuid,
    pub is_public: bool,
    pub name: String,
    pub description: String,
    pub homepage_url: Url,
    pub redirect_url: Url,
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = clients)]
pub struct ClientUpdateModel {
    pub name: Option<String>,
    pub description: Option<String>,
    pub homepage_url: Option<String>,
}
