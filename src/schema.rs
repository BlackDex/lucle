// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        email -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        role -> Text,
        reset_token -> Nullable<Text>,
    }
}
