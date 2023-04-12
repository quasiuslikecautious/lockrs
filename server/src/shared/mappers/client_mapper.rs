use crate::{db::models::DbClient, models::ClientModel};

pub struct ClientMapper;

impl ClientMapper {
    pub fn from_db(db_client: DbClient) -> ClientModel {
        ClientModel {
            id: db_client.id,
            secret: db_client.secret,
            name: db_client.name,
            description: db_client.description,
            homepage_url: db_client.homepage_url,
        }
    }
}
