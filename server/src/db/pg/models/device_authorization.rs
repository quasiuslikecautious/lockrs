use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::pg::schema::device_authorizations;

#[derive(Debug, Queryable, Insertable, Identifiable)]
#[diesel(primary_key(id), table_name = device_authorizations)]
pub struct PgDeviceAuthorization {
    pub id: i32,
    pub client_id: String,
    pub user_code: String,
    pub device_code: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub scopes: Vec<Option<String>>,
}
