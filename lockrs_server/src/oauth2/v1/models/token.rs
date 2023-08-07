use uuid::Uuid;

pub struct TokenModel {
    pub token_type: String,
    pub expires_in: i64,
    pub access_token: String,
    pub refresh_token: String,
    pub scopes: String,
}

impl TokenModel {
    pub fn new(
        token_type: &str,
        expires_in: i64,
        access_token: &str,
        refresh_token: &str,
        scopes: &str,
    ) -> Self {
        Self {
            token_type: token_type.to_owned(),
            expires_in,
            access_token: access_token.to_owned(),
            refresh_token: refresh_token.to_owned(),
            scopes: scopes.to_owned(),
        }
    }
}

impl std::fmt::Debug for TokenModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TokenModel: {{ {:?}, {:?}, access_token: ********, refresh_token: ********, {:?} }}",
            self.token_type, self.expires_in, self.scopes,
        )
    }
}

#[derive(Debug)]
pub struct TokenCreateModel {
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub scopes: Vec<String>,
}

impl TokenCreateModel {
    pub fn new(client_id: &str, user_id: Option<&Uuid>, scopes: &[String]) -> Self {
        Self {
            client_id: client_id.to_owned(),
            user_id: user_id.map(|id| id.to_owned()),
            scopes: scopes.to_vec(),
        }
    }
}
