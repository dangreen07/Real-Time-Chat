// @generated automatically by Diesel CLI.

diesel::table! {
    contacts (id) {
        id -> Uuid,
        user_id -> Uuid,
        contact_id -> Uuid,
    }
}

diesel::table! {
    messages (id) {
        id -> Uuid,
        user_id -> Uuid,
        recipient_id -> Uuid,
        message -> Varchar,
        sent_at -> Timestamp,
    }
}

diesel::table! {
    sessions (id) {
        id -> Uuid,
        expiry -> Timestamp,
        user_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password_hash -> Varchar,
        permission -> Varchar,
        full_name -> Varchar,
    }
}

diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    contacts,
    messages,
    sessions,
    users,
);
