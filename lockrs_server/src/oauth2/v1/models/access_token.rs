use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(PartialEq)]
pub struct AccessTokenModel {
    pub id: i32,
    pub token: String,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub expires_at: NaiveDateTime,
    pub scopes: Vec<String>,
}

impl AccessTokenModel {
    pub fn new(
        id: i32,
        token: &str,
        client_id: &str,
        user_id: Option<&Uuid>,
        expires_at: &NaiveDateTime,
        scopes: &[String],
    ) -> Self {
        Self {
            id,
            token: token.to_owned(),
            client_id: client_id.to_owned(),
            user_id: user_id.map(|u| u.to_owned()),
            expires_at: expires_at.to_owned(),
            scopes: scopes.to_vec(),
        }
    }
}

impl std::fmt::Debug for AccessTokenModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AccessTokenModel: {{ {:?}, token: ********, {:?}, {:?}, {:?}, {:?} }}",
            self.id, self.client_id, self.user_id, self.expires_at, self.scopes,
        )
    }
}

pub struct AccessTokenCreateModel {
    pub token: String,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub expires_at: NaiveDateTime,
    pub scopes: Vec<String>,
}

impl AccessTokenCreateModel {
    pub fn new(
        token: &str,
        client_id: &str,
        user_id: Option<&Uuid>,
        expires_at: &NaiveDateTime,
        scopes: &[String],
    ) -> Self {
        Self {
            token: token.to_owned(),
            client_id: client_id.to_owned(),
            user_id: user_id.map(|u| u.to_owned()),
            expires_at: expires_at.to_owned(),
            scopes: scopes.to_vec(),
        }
    }
}

impl std::fmt::Debug for AccessTokenCreateModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AccessTokenCreateModel: {{ token: ********, {:?}, {:?}, {:?}, {:?} }}",
            self.client_id, self.user_id, self.expires_at, self.scopes,
        )
    }
}
