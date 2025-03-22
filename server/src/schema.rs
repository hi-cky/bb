// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Integer,
        sender -> Integer,
        receiver -> Integer,
        #[max_length = 50]
        message_type -> Varchar,
        content -> Text,
        timestamp -> Datetime,
        dead_time -> Datetime,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        status -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    users,
);
