use url::Url;
use uuid::Uuid;

pub struct ClientCreateModel {
    pub user_id: Uuid,
    pub is_public: bool,
    pub name: String,
    pub description: String,
    pub homepage_url: Url,
    pub redirect_url: Url,
}
