use serde::Serialize;

#[derive(Serialize)]
pub struct Session {
    pub id: String,
    pub token: String,
}

