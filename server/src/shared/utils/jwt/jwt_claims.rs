use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims<T> {
    #[serde(flatten)]
    pub claims: T,
    pub version: Uuid,
}
