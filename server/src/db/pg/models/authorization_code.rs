use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::pg::schema::authorization_codes;

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = authorization_codes)]
pub struct PgAuthorizationCode {
    pub id: i32,
    pub code: String,
    pub challenge: String,
    pub is_challenge_plain: bool,
    pub client_id: String,
    pub user_id: Uuid,
    pub redirect_uri: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub used: bool,
    pub scopes: Vec<Option<String>>,
}
