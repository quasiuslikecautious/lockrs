use crate::{
    db::pg::models::PgClient,
    models::{ClientAuthModel, ClientModel},
};

pub struct ClientAuthMapper;

impl ClientAuthMapper {
    pub fn from_pg(pg_client: PgClient) -> ClientAuthModel {
        ClientAuthModel::new(
            &pg_client.user_id,
            pg_client.id.as_str(),
            pg_client.secret.as_deref(),
            pg_client.name.as_str(),
            pg_client.description.as_str(),
            pg_client.homepage_url.as_str(),
        )
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
