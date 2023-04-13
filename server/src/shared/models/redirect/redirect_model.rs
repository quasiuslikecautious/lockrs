use url::Url;

#[derive(Debug, PartialEq)]
pub struct RedirectModel {
    pub id: i32,
    pub client_id: String,
    pub uri: Url,
}
