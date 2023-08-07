use chrono::NaiveDateTime;
use url::Url;
use uuid::Uuid;

pub struct AuthorizationCodeModel {
    pub client_id: String,
    pub user_id: Uuid,
    pub code: String,
    pub challenge: String,
    pub is_challenge_plain: bool,
    pub redirect_uri: Url,
    pub expires_at: NaiveDateTime,
    pub scopes: Vec<String>,
}

impl AuthorizationCodeModel {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        client_id: &str,
        user_id: &Uuid,
        code: &str,
        challenge: &str,
        is_challenge_plain: bool,
        redirect_uri: &Url,
        expires_at: &NaiveDateTime,
        scopes: &[String],
    ) -> Self {
        Self {
            client_id: client_id.to_owned(),
            user_id: user_id.to_owned(),
            code: code.to_owned(),
            challenge: challenge.to_owned(),
            is_challenge_plain,
            redirect_uri: redirect_uri.to_owned(),
            expires_at: expires_at.to_owned(),
            scopes: scopes.to_vec(),
        }
    }
}

impl std::fmt::Debug for AuthorizationCodeModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AuthorizationCodeModel: {{ {:?}, {:?}, code: ********, challenge: ********, {:?}, {:?}, {:?}, {:?} }}",
            self.client_id,
            self.user_id,
            self.is_challenge_plain,
            self.redirect_uri,
            self.scopes,
            self.expires_at,
        )
    }
}

pub struct AuthorizationCodeCreateModel {
    client_id: String,
    user_id: Uuid,
    challenge: String,
    is_challenge_plain: bool,
    redirect_uri: Url,
    scopes: Vec<String>,
}

impl AuthorizationCodeCreateModel {
    pub fn new(
        client_id: &str,
        user_id: &Uuid,
        challenge: &str,
        is_challenge_plain: bool,
        redirect_uri: &Url,
        scopes: &[String],
    ) -> Self {
        Self {
            client_id: client_id.to_owned(),
            user_id: user_id.to_owned(),
            challenge: challenge.to_owned(),
            is_challenge_plain,
            redirect_uri: redirect_uri.to_owned(),
            scopes: scopes.to_vec(),
        }
    }
}

impl std::fmt::Debug for AuthorizationCodeCreateModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AuthorizationCodeModel: {{ {:?}, {:?}, challenge: ********, {:?}, {:?}, {:?} }}",
            self.client_id, self.user_id, self.is_challenge_plain, self.redirect_uri, self.scopes,
        )
    }
}
