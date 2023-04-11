use url::Url;

pub struct RedirectUri {
   pub id: i32,
   pub client_id: String,
   pub uri: Url,
}

