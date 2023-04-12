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
    pub scopes: Vec<String>,
    pub expires_at: NaiveDateTime,
}
