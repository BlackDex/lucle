diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        email -> Text,
        createdAt -> Text,
        modifiedAt -> Text,
        privilege -> Text
    }
}
