use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub id: Uuid,
    pub token: String,
}

