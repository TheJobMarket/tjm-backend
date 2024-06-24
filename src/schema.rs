// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    posts,
);
