diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        email -> Text,
        createdat -> Text,
        modifiedat -> Text,
        privilege -> Text
    }
}
