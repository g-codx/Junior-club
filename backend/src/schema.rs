// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Int4,
        title -> Varchar,
        link -> Varchar,
        link_type -> Varchar,
        section_type -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        preview -> Text,
        body -> Text,
        published -> Bool,
        public_date -> Nullable<Timestamp>,
        search_tag -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        unique_id -> Varchar,
        role -> Varchar,
    }
}

diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    links,
    posts,
    users,
);
