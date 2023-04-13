use crate::{db::models::DbClient, models::ClientModel};

pub struct ClientMapper;

impl ClientMapper {
    pub fn from_db(db_client: DbClient) -> ClientModel {
        ClientModel {
            user_id: db_client.user_id,
            id: db_client.id,
            secret: db_client.secret,
            name: db_client.name,
            description: db_client.description,
            homepage_url: db_client.homepage_url,
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn it_should_map_db_confidential_client() {
        let id = String::from("CLIENT_ID");
        let secret = Some(String::from("CLIENT_SECRET"));
        let user_id = Uuid::new_v4();
        let name = String::from("CLIENT_NAME");
        let description = String::from("CLIENT_DESCRIPTION");
        let homepage_url = String::from("https://127.0.0.1/");

        let db_client = DbClient {
            id: id.clone(),
            secret: secret.clone(),
            user_id,
            is_public: false,
            name: name.clone(),
            description: description.clone(),
            homepage_url: homepage_url.clone(),
        };

        let actual_client = ClientMapper::from_db(db_client);

        let expected_client = ClientModel {
            user_id,
            id,
            secret,
            name,
            description,
            homepage_url,
        };

        assert_eq!(actual_client, expected_client);
    }

    #[test]
    fn it_should_map_db_public_client() {
        let id = String::from("CLIENT_ID");
        let secret = None;
        let user_id = Uuid::new_v4();
        let name = String::from("CLIENT_NAME");
        let description = String::from("CLIENT_DESCRIPTION");
        let homepage_url = String::from("https://127.0.0.1/");

        let db_client = DbClient {
            id: id.clone(),
            secret: secret.clone(),
            user_id,
            is_public: true,
            name: name.clone(),
            description: description.clone(),
            homepage_url: homepage_url.clone(),
        };

        let actual_client = ClientMapper::from_db(db_client);

        let expected_client = ClientModel {
            user_id,
            id,
            secret,
            name,
            description,
            homepage_url,
        };

        assert_eq!(actual_client, expected_client);
    }
}
