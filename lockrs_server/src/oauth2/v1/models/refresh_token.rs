use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(PartialEq)]
pub struct RefreshTokenModel {
    pub id: i32,
    pub access_token_id: i32,
    pub token: String,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub expires_at: NaiveDateTime,
    pub scopes: Vec<String>,
}

impl RefreshTokenModel {
    pub fn new(
        id: i32,
        access_token_id: i32,
        token: &str,
        client_id: &str,
        user_id: Option<&Uuid>,
        expires_at: &NaiveDateTime,
        scopes: &[String],
    ) -> Self {
        Self {
            id,
            access_token_id,
            token: token.to_owned(),
            client_id: client_id.to_owned(),
            user_id: user_id.map(|u| u.to_owned()),
            expires_at: expires_at.to_owned(),
            scopes: scopes.to_vec(),
        }
    }
}

impl std::fmt::Debug for RefreshTokenModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RefreshTokenModel: {{ {:?}, {:?}, token: ********, {:?}, {:?}, {:?}, {:?} }}",
            self.id,
            self.access_token_id,
            self.client_id,
            self.user_id,
            self.expires_at,
            self.scopes,
        )
    }
}

pub struct RefreshTokenCreateModel {
    pub token: String,
    pub access_token_id: i32,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub expires_at: NaiveDateTime,
    pub scopes: Vec<String>,
}

impl RefreshTokenCreateModel {
    pub fn new(
        token: &str,
        access_token_id: i32,
        client_id: &str,
        user_id: Option<&Uuid>,
        expires_at: &NaiveDateTime,
        scopes: &[String],
    ) -> Self {
        Self {
            token: token.to_owned(),
            access_token_id,
            client_id: client_id.to_owned(),
            user_id: user_id.map(|u| u.to_owned()),
            expires_at: expires_at.to_owned(),
            scopes: scopes.to_vec(),
        }
    }
}

impl std::fmt::Debug for RefreshTokenCreateModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RefreshTokenCreateModel: {{ token: ********, {:?}, {:?}, {:?}, {:?}, {:?} }}",
            self.access_token_id, self.client_id, self.user_id, self.expires_at, self.scopes,
        )
    }
}
