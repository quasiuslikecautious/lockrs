use uuid::Uuid;

pub struct TokenCreateModel {
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub scopes: Vec<String>,
}
