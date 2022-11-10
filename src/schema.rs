// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Int4,
        slug -> Text,
        title -> Text,
        description -> Text,
        body -> Text,
        author -> Int4,
        tag_list -> Array<Nullable<Text>>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        favorites_count -> Int4,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        body -> Text,
        article -> Int4,
        author -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    favorites (user, article) {
        user -> Int4,
        article -> Int4,
    }
}

diesel::table! {
    follows (follower, followed) {
        follower -> Int4,
        followed -> Int4,
    }
}

diesel::table! {
    tasks (id) {
        id -> Int4,
        description -> Varchar,
        completed -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
        hash -> Text,
    }
}

diesel::joinable!(articles -> users (author));
diesel::joinable!(comments -> articles (article));
diesel::joinable!(comments -> users (author));
diesel::joinable!(favorites -> articles (article));
diesel::joinable!(favorites -> users (user));

diesel::allow_tables_to_appear_in_same_query!(
    articles,
    comments,
    favorites,
    follows,
    tasks,
    users,
);
