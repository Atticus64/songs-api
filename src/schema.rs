// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "category"))]
    pub struct Category;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Category;

    songs (id) {
        id -> Int4,
        title -> Text,
        category -> Category,
        interpreter -> Nullable<Text>,
        num_verses -> Int4,
    }
}

diesel::table! {
    verses (id) {
        id -> Int4,
        content -> Text,
        song_id -> Int4,
    }
}

diesel::joinable!(verses -> songs (song_id));

diesel::allow_tables_to_appear_in_same_query!(songs, verses,);
