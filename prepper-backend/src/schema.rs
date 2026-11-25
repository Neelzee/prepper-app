// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Int4,
        user_id -> Int4,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
        last_seen_at -> Timestamptz,
        user_agent -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        pswd -> Nullable<Text>,
    }
}

diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(sessions, users,);
