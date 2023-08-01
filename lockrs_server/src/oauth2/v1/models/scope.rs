pub struct ScopeCreateModel {
    pub client_id: String,
    pub scope: String,
}

#[derive(Clone, Debug)]
pub struct ScopeModel {
    pub scopes: Vec<String>,
}
