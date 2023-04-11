use serde::Serialize;
use url::Url;

#[derive(Serialize)]
pub struct RedirectResponse {
    pub id: i32,
    pub client_id: String,
    pub uri: Url,
}
