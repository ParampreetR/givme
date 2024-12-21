diesel::table! {
    credentials (id) {
        id -> Integer,
        key -> Text,
        value -> Text,
        info -> Nullable<Text>,
    }
}
