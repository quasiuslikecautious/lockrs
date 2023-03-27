use std::env;

use chrono;
use dotenvy::dotenv;
use diesel::{
    pg::PgConnection,
    prelude::*,
};
use uuid::Uuid;

use crate::schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = schema::access_tokens)]
pub struct DbAccessToken {
    pub id: i32,
    pub token: String,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
    pub scopes: Vec<Option<String>>,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = schema::authorization_codes)]
pub struct DbAuthorizationCode {
    pub id: i32,
    pub code: String,
    pub challenge: String,
    pub is_challenge_plain: bool,
    pub client_id: String,
    pub user_id: Uuid,
    pub redirect_uri: String,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
    pub used: bool,
    pub scopes: Vec<Option<String>>,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = schema::clients)]
pub struct DbClient {
    pub id: String,
    pub secret: Option<String>,
    pub user_id: Uuid,
    pub redirect_uri: String,
    pub is_public: bool,
    pub name: String,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = schema::device_codes)]
pub struct DbDeviceCode {
   pub id: i32,
   pub client_id: String,
   pub user_code: String,
   pub device_code: String,
   pub created_at: chrono::NaiveDateTime,
   pub expires_at: chrono::NaiveDateTime,
   pub scopes: Vec<String>,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = schema::redirect_uris)]
pub struct DbRedirectUri {
    pub id: i32,
    pub client_id: String,
    pub uri: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = schema::refresh_tokens)]
pub struct DbRefreshToken {
    pub id: i32,
    pub token: String,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
    pub used: bool,
    pub scopes: Vec<Option<String>>,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = schema::scopes)]
pub struct DbScope {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub client_id: Option<String>,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = schema::users)]
pub struct DbUser {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

