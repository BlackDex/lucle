diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password -> Text,
        email -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        privilege -> Text,
        reset_token -> Nullable<Text>,
    }
}
