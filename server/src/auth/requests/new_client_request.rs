use serde::Serialize;
use url::Url;

#[derive(Serialize)]
pub struct NewClientRequest {
    pub name: String,
    pub description: String,
    pub homepage_url: Url,
    pub redirect_url: Url,
}
