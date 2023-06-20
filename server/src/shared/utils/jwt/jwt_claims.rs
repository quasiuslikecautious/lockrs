use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims<T> {
    #[serde(flatten)]
    pub claims: T,
    pub iat: i64,
    pub nbf: i64,
    pub exp: i64,
    pub rev: Uuid,
}
