use serde::Serialize;

use super::ClientResponse;

#[derive(Serialize)]
pub struct ClientListResponse {
    pub clients: Vec<ClientResponse>,
}
