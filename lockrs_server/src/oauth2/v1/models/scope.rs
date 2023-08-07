use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct ScopeModel {
    data: Vec<String>,
}

impl ScopeModel {
    pub fn new(scopes: &[String]) -> Self {
        Self {
            data: scopes.to_vec(),
        }
    }
}

impl Deref for ScopeModel {
    type Target = [String];

    fn deref(&self) -> &[String] {
        self.data.as_slice()
    }
}

#[derive(Debug)]
pub struct ScopeCreateModel {
    pub client_id: String,
    pub scope: String,
}

impl ScopeCreateModel {
    pub fn new(client_id: &str, scope: &str) -> Self {
        Self {
            client_id: client_id.to_owned(),
            scope: scope.to_owned(),
        }
    }
}
