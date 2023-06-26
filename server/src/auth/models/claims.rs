use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub iss: String,
    pub aud: Option<String>,

    pub iat: i64,
    pub nbf: i64,
    pub exp: i64,
}
