// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password -> Text,
        email -> Text,
        created_at -> Text,
        modified_at -> Text,
        privilege -> Text,
        reset_token -> Nullable<Text>,
    }
}
