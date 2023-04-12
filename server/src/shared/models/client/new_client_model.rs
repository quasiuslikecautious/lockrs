use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
pub struct NewClientModel {
    pub is_public: bool,
    pub name: String,
    pub description: String,
    pub homepage_url: Url,
    pub redirect_url: Url,
}

