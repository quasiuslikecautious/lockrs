use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::pg::schema::access_tokens;

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = access_tokens)]
pub struct PgAccessToken {
    pub id: i32,
    pub token: String,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub scopes: Vec<Option<String>>,
}
