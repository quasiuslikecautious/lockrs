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

impl ClientModel {
    pub fn new(
        user_id: &Uuid,
        id: &str,
        is_public: bool,
        name: &str,
        description: &str,
        homepage_url: &str,
    ) -> Self {
        Self {
            user_id: user_id.to_owned(),
            id: id.to_owned(),
            is_public,
            name: name.to_owned(),
            description: description.to_owned(),
            homepage_url: homepage_url.to_owned(),
        }
    }
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = clients)]
pub struct ClientUpdateModel {
    pub name: Option<String>,
    pub description: Option<String>,
    pub homepage_url: Option<String>,
}

impl ClientUpdateModel {
    pub fn new(name: Option<&str>, description: Option<&str>, homepage_url: Option<&str>) -> Self {
        Self {
            name: name.map(|s| s.to_owned()),
            description: description.map(|s| s.to_owned()),
            homepage_url: homepage_url.map(|s| s.to_owned()),
        }
    }
}
