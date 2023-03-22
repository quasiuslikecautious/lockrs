// @generated automatically by Diesel CLI.

diesel::table! {
    access_tokens (id) {
        id -> Int4,
        token -> Varchar,
        client_id -> Varchar,
        user_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        scopes -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    authorization_codes (id) {
        id -> Int4,
        code -> Varchar,
        challenge -> Varchar,
        is_challenge_plain -> Bool,
        client_id -> Varchar,
        user_id -> Uuid,
        redirect_uri -> Text,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        used -> Bool,
    }
}

diesel::table! {
    clients (id) {
        id -> Varchar,
        secret -> Nullable<Text>,
        redirect_uri -> Text,
        is_public -> Bool,
        name -> Text,
    }
}

diesel::table! {
    device_codes (id) {
        id -> Int4,
        client_id -> Varchar,
        user_code -> Varchar,
        device_code -> Varchar,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        scopes -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    redirect_uris (id) {
        id -> Int4,
        client_id -> Varchar,
        uri -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    refresh_tokens (id) {
        id -> Int4,
        token -> Varchar,
        client_id -> Varchar,
        user_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        used -> Bool,
        scopes -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password_hash -> Varchar,
    }
}

diesel::joinable!(access_tokens -> clients (client_id));
diesel::joinable!(access_tokens -> users (user_id));
diesel::joinable!(authorization_codes -> clients (client_id));
diesel::joinable!(authorization_codes -> users (user_id));
diesel::joinable!(device_codes -> clients (client_id));
diesel::joinable!(redirect_uris -> clients (client_id));
diesel::joinable!(refresh_tokens -> clients (client_id));
diesel::joinable!(refresh_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_tokens,
    authorization_codes,
    clients,
    device_codes,
    redirect_uris,
    refresh_tokens,
    users,
);
