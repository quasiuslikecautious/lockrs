use url::Url;
use uuid::Uuid;

pub struct AuthorizationCodeCreateModel {
    client_id: String,
    user_id: Uuid,
    challenge: String,
    is_challenge_plain: bool,
    redirect_uri: Url,
    scopes: Vec<String>,
}
