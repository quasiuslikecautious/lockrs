use url::Url;

pub struct RedirectModel {
    pub id: i32,
    pub client_id: String,
    pub uri: Url,
}
