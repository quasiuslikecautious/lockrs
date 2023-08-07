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
        ClientModel::new(
            &client_auth.user_id,
            client_auth.id.as_str(),
            client_auth.secret.is_none(),
            client_auth.name.as_str(),
            client_auth.description.as_str(),
            client_auth.homepage_url.as_str(),
        )
    }
}
