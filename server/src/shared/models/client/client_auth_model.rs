#[derive(Debug)]
pub struct ClientAuthModel {
    pub id: String,
    pub secret: Option<String>,
}
