use url::Url;
use uuid::Uuid;

pub struct ClientAuthModel {
    pub user_id: Uuid,
    pub id: String,
    pub secret: Option<String>,
    pub name: String,
    pub description: String,
    pub homepage_url: String,
}

impl ClientAuthModel {
    pub fn new(
        user_id: &Uuid,
        id: &str,
        secret: Option<&str>,
        name: &str,
        description: &str,
        homepage_url: &str,
    ) -> Self {
        Self {
            user_id: user_id.to_owned(),
            id: id.to_owned(),
            secret: secret.map(|s| s.to_owned()),
            name: name.to_owned(),
            description: description.to_owned(),
            homepage_url: homepage_url.to_owned(),
        }
    }
}

impl std::fmt::Debug for ClientAuthModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ClientAuthModel: {{ {:?}, {:?}, secret: ********, {:?}, {:?}, {:?} }}",
            self.user_id,
            self.id,
            self.name,
            self.description,
            self.homepage_url,
        )
    }
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

impl ClientRegistration {
    pub fn new(
        user_id: &Uuid,
        is_public: bool,
        name: &str,
        description: &str,
        redirect_url: &Url,
        homepage_url: &Url,
    ) -> Self {
        Self {
            user_id: user_id.to_owned(),
            is_public,
            name: name.to_owned(),
            description: description.to_owned(),
            redirect_url: redirect_url.to_owned(),
            homepage_url: homepage_url.to_owned(),
        }
    }
}

pub struct ClientLoginCredentials {
    pub id: String,
    pub secret: Option<String>,
}

impl ClientLoginCredentials {
    pub fn new(id: &str, secret: Option<&str>) -> Self {
        Self {
            id: id.to_owned(),
            secret: secret.map(|s| s.to_owned()),
        }
    }
}

impl std::fmt::Debug for ClientLoginCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClientLoginCredentials: {{ {:?}, secret: ******** }}", self.id)
    }
}
