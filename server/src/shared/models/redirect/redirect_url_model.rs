use url::Url;

pub struct RedirectUriModel {
   pub id: i32,
   pub client_id: String,
   pub uri: Url,
}

