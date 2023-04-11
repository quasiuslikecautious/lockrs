use serde::Serialize;

#[derive(Serialize)]
struct ClientResponse {
    pub id: String,
    pub is_public: bool,
    pub name: String,
    pub description: String,
    pub homepage_url: String,
    pub redirect_urls: Vec<String>,
}
