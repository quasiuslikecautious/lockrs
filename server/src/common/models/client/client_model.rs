use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct ClientModel {
    pub user_id: Uuid,
    pub id: String,
    pub secret: Option<String>,
    pub name: String,
    pub description: String,
    pub homepage_url: String,
}
