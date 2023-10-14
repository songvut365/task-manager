diesel::table! {
    task (id) {
        id -> Text,
        title -> Text,
        description -> Text,
        completed -> Bool,
        owner -> Text
    }
}
