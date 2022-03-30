table! {
    bgimgs (id) {
        id -> Integer,
        day -> Integer,
        device -> Text,
        url -> Text,
    }
}

table! {
    says (id) {
        id -> Integer,
        saying -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    bgimgs,
    says,
);
