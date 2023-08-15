use axum::{Json, extract::Query};
use diesel::{RunQueryDsl, QueryDsl, SelectableHelper, TextExpressionMethods, sql_function};
use serde::Deserialize;

use crate::models::Vers;
use crate::{db::establish_connection, models::Song};
use crate::schema::songs::dsl::*;
use crate::schema::verses::dsl::*;

pub async fn get_songs() -> Json<Vec<Song>> {


    
    let mut conn = establish_connection();

    match songs.load::<Song>(&mut conn) {
        Ok(data) => Json(data),
        Err(_) => Json([].to_vec())
    }
}

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<usize>,
    q: String,
    per_page: Option<usize>,
}


pub async fn search_song(query: Query<Pagination>) -> Json<Vec<(Vers, Song)>> {

    let mut connection = establish_connection();
    sql_function!(fn lower(a: diesel::sql_types::VarChar) -> diesel::sql_types::VarChar);
    sql_function!(fn unaccent(a: diesel::sql_types::VarChar) -> diesel::sql_types::VarChar);
    let data = songs.inner_join(verses)
        .select((Vers::as_select(), Song::as_select()))
        .filter(unaccent(lower(content)).like(format!("%{}%", query.q)))
        .distinct_on(content)
        .load::<(Vers, Song)>(&mut connection).unwrap();

    Json(data)
}
