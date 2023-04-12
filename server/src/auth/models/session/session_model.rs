use serde::Serialize;

#[derive(Serialize)]
pub struct SessionModel {
    pub id: String,
    pub token: String,
}
