use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub id: String,
    pub token: String,
}

