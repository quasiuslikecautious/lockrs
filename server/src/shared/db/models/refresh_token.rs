use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::schema::refresh_tokens;

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = refresh_tokens)]
pub struct DbRefreshToken {
    pub id: i32,
    pub token: String,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub used: bool,
    pub scopes: Vec<Option<String>>,
}
