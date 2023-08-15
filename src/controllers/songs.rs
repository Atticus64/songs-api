use axum::Json;
use diesel::RunQueryDsl;

use crate::{db::establish_connection, models::Song};

pub async fn get_songs() -> Json<Vec<Song>> {

    use crate::schema::songs::dsl::*;

    
    let mut conn = establish_connection();

    match songs.load::<Song>(&mut conn) {
        Ok(data) => Json(data),
        Err(_) => Json([].to_vec())
    }
}

