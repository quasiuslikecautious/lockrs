use serde::Serialize;

#[derive(Serialize)]
pub struct ClientResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub homepage_url: String,
}

