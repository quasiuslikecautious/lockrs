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
    id: i32,
    token: String,
    client_id: Uuid,
    user_id: Option<Uuid>,
    created_at: chrono::NaiveDateTime,
    expires_at: chrono::NaiveDateTime,
    scopes: Vec<String>,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = schema::authorization_codes)]
pub struct DbAuthorizationCode {
    id: i32,
    code: String,
    client_id: Uuid,
    redirect_uri: String,
    user_id: Uuid,
    created_at: chrono::NaiveDateTime,
    expires_at: chrono::NaiveDateTime,
    used: bool,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = schema::clients)]
pub struct DbClient {
    pub id: Uuid,
    pub secret: Option<String>,
    pub redirect_uri: String,
    pub is_public: bool,
    pub name: String,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = schema::device_codes)]
pub struct DbDeviceCodes {
    pub id: i32,
    pub client_id: Uuid,
    pub user_code: String,
    pub device_code: String,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
    pub scopes: Vec<String>,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = schema::redirect_uris)]
pub struct DbRedirectUri {
    id: i32,
    client_id: Uuid,
    uri: String,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = schema::refresh_tokens)]
pub struct DbRefreshToken {
    id: i32,
    token: String,
    client_id: Uuid,
    user_id: Option<Uuid>,
    created_at: chrono::NaiveDateTime,
    expires_at: chrono::NaiveDateTime,
    used: bool,
    scopes: Vec<String>,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = schema::users)]
pub struct DbUser {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

