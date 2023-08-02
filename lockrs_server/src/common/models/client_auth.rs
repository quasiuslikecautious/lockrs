use url::Url;
use uuid::Uuid;

#[derive(Debug)]
pub struct ClientAuthModel {
    pub user_id: Uuid,
    pub id: String,
    pub secret: Option<String>,
    pub name: String,
    pub description: String,
    pub homepage_url: String,
}

#[derive(Debug)]
pub struct ClientRegistration {
    pub user_id: Uuid,
    pub is_public: bool,
    pub name: String,
    pub description: String,
    pub redirect_url: Url,
    pub homepage_url: Url,
}

#[derive(Debug)]
pub struct ClientLoginCredentials {
    pub id: String,
    pub secret: Option<String>,
}
