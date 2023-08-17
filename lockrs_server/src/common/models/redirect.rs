use url::Url;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct RedirectModel {
    pub id: Uuid,
    pub client_id: String,
    pub uri: Url,
}

impl RedirectModel {
    pub fn new(id: &Uuid, client_id: &str, uri: &Url) -> Self {
        Self {
            id: id.to_owned(),
            client_id: client_id.to_owned(),
            uri: uri.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct RedirectCreateModel {
    pub client_id: String,
    pub uri: Url,
}

impl RedirectCreateModel {
    pub fn new(client_id: &str, uri: &Url) -> Self {
        Self {
            client_id: client_id.to_owned(),
            uri: uri.to_owned(),
        }
    }
}

// TODO
pub struct RedirectUpdateModel {}
