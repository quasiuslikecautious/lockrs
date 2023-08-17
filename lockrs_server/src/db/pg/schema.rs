// @generated automatically by Diesel CLI.

diesel::table! {
    access_tokens (id) {
        id -> Int4,
        #[max_length = 128]
        token -> Varchar,
        #[max_length = 32]
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
        #[max_length = 100]
        code -> Varchar,
        #[max_length = 128]
        challenge -> Varchar,
        is_challenge_plain -> Bool,
        #[max_length = 32]
        client_id -> Varchar,
        user_id -> Uuid,
        redirect_uri -> Text,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        used -> Bool,
        scopes -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    clients (id) {
        #[max_length = 32]
        id -> Varchar,
        #[max_length = 32]
        secret -> Nullable<Varchar>,
        user_id -> Uuid,
        is_public -> Bool,
        name -> Text,
        #[max_length = 300]
        description -> Varchar,
        homepage_url -> Text,
    }
}

diesel::table! {
    device_authorizations (id) {
        id -> Int4,
        #[max_length = 32]
        client_id -> Varchar,
        #[max_length = 8]
        user_code -> Varchar,
        #[max_length = 44]
        device_code -> Varchar,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        scopes -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    redirect_uris (id) {
        id -> Uuid,
        #[max_length = 32]
        client_id -> Varchar,
        uri -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    refresh_tokens (id) {
        id -> Int4,
        access_token_id -> Int4,
        #[max_length = 44]
        token -> Varchar,
        #[max_length = 32]
        client_id -> Varchar,
        user_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        used -> Bool,
        scopes -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    scopes (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        #[max_length = 32]
        client_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 256]
        email -> Varchar,
        #[max_length = 256]
        password_hash -> Varchar,
    }
}

diesel::joinable!(access_tokens -> clients (client_id));
diesel::joinable!(access_tokens -> users (user_id));
diesel::joinable!(authorization_codes -> clients (client_id));
diesel::joinable!(authorization_codes -> users (user_id));
diesel::joinable!(clients -> users (user_id));
diesel::joinable!(device_authorizations -> clients (client_id));
diesel::joinable!(redirect_uris -> clients (client_id));
diesel::joinable!(refresh_tokens -> access_tokens (access_token_id));
diesel::joinable!(refresh_tokens -> clients (client_id));
diesel::joinable!(refresh_tokens -> users (user_id));
diesel::joinable!(scopes -> clients (client_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_tokens,
    authorization_codes,
    clients,
    device_authorizations,
    redirect_uris,
    refresh_tokens,
    scopes,
    users,
);
