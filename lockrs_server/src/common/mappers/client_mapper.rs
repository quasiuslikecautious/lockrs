use crate::{db::pg::models::PgClient, models::ClientModel};

pub struct ClientMapper;

impl ClientMapper {
    pub fn from_pg(pg_client: PgClient) -> ClientModel {
        ClientModel::new(
            &pg_client.user_id,
            pg_client.id.as_str(),
            pg_client.secret.is_none(),
            pg_client.name.as_str(),
            pg_client.description.as_str(),
            pg_client.homepage_url.as_str(),
        )
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn it_should_map_pg_confidential_client() {
        let id = String::from("CLIENT_ID");
        let secret = Some(String::from("CLIENT_SECRET"));
        let user_id = Uuid::new_v4();
        let name = String::from("CLIENT_NAME");
        let description = String::from("CLIENT_DESCRIPTION");
        let homepage_url = String::from("https://127.0.0.1/");

        let pg_client = PgClient {
            id: id.clone(),
            secret,
            user_id,
            is_public: false,
            name: name.clone(),
            description: description.clone(),
            homepage_url: homepage_url.clone(),
        };

        let actual_client = ClientMapper::from_pg(pg_client);

        let expected_client = ClientModel::new(
            &user_id,
            id.as_str(),
            false,
            name.as_str(),
            description.as_str(),
            homepage_url.as_str(),
        );

        assert_eq!(actual_client, expected_client);
    }

    #[test]
    fn it_should_map_pg_public_client() {
        let id = String::from("CLIENT_ID");
        let secret = None;
        let user_id = Uuid::new_v4();
        let name = String::from("CLIENT_NAME");
        let description = String::from("CLIENT_DESCRIPTION");
        let homepage_url = String::from("https://127.0.0.1/");

        let pg_client = PgClient {
            id: id.clone(),
            secret,
            user_id,
            is_public: true,
            name: name.clone(),
            description: description.clone(),
            homepage_url: homepage_url.clone(),
        };

        let actual_client = ClientMapper::from_pg(pg_client);

        let expected_client = ClientModel::new(
            &user_id,
            id.as_str(),
            true,
            name.as_str(),
            description.as_str(),
            homepage_url.as_str(),
        );

        assert_eq!(actual_client, expected_client);
    }
}
