use crate::{
    db::pg::models::PgClient,
    models::{ClientAuthModel, ClientModel},
};

pub struct ClientAuthMapper;

impl ClientAuthMapper {
    pub fn from_pg(pg_client: PgClient) -> ClientAuthModel {
        ClientAuthModel {
            user_id: pg_client.user_id,
            id: pg_client.id,
            secret: pg_client.secret,
            name: pg_client.name,
            description: pg_client.description,
            homepage_url: pg_client.homepage_url,
        }
    }

    pub fn into_client(client_auth: ClientAuthModel) -> ClientModel {
        ClientModel {
            user_id: client_auth.user_id,
            id: client_auth.id,
            is_public: client_auth.secret.is_none(),
            name: client_auth.name,
            description: client_auth.description,
            homepage_url: client_auth.homepage_url,
        }
    }
}
