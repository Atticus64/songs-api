use axum::{Json, extract::Query};
use diesel::{RunQueryDsl, QueryDsl, SelectableHelper, TextExpressionMethods, sql_function};
use serde::{Deserialize, Serialize};

use crate::models::Vers;
use crate::{db::establish_connection, models::Song};
use crate::schema::songs::dsl::*;
use crate::schema::verses::dsl::*;

sql_function!(fn lower(a: diesel::sql_types::VarChar) -> diesel::sql_types::VarChar);
sql_function!(fn unaccent(a: diesel::sql_types::VarChar) -> diesel::sql_types::VarChar);

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

#[derive(Deserialize, Serialize)]
pub struct ResultSearch {
    data: Vec<(Vers, Song)>,
    count: i64,
    page: usize,
    items: usize,
    total_pages: usize,
    per_page: usize
}

pub async fn search_song(query: Query<Pagination>) -> Json<ResultSearch> {

    let mut connection = establish_connection();

    let page = query.page.unwrap_or(1) - 1;

    let per_page = query.per_page.unwrap_or(10);

    let offset = page * per_page;

    let q = query.q.to_lowercase();

    let count: i64 = songs.inner_join(verses)
        .filter(unaccent(lower(content)).like(format!("%{}%", q)))
        .count()
        .get_result(&mut connection).unwrap();


    let data = songs.inner_join(verses)
        .select((Vers::as_select(), Song::as_select()))
        .filter(unaccent(lower(content)).like(format!("%{}%", q)))
        .limit(per_page as i64)
        .offset(offset as i64)
        .load::<(Vers, Song)>(&mut connection).unwrap();

    let len = data.len();

    Json(ResultSearch {
        data,
        count,
        items: len,
        page: page + 1,
        per_page, 
        total_pages: (count as usize / per_page) + 1
    })
}

