use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ClientIdQueryParam {
    pub client_id: Uuid,
}

#[derive(Deserialize)]
pub struct GrantTypeQueryParam {
    pub grant_type: String,
}
