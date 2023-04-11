use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::db::{
    models::DbClient,
    schema::redirect_uris,
};

#[derive(Debug, Queryable, Insertable, Associations, Identifiable)]
#[diesel(belongs_to(DbClient, foreign_key = client_id))]
#[diesel(primary_key(id), table_name = redirect_uris)]
pub struct DbRedirectUri {
    pub id: i32,
    pub client_id: String,
    pub uri: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

