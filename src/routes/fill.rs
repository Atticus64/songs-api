use diesel::{insert_into, Connection, PgConnection, RunQueryDsl};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::{env, fs};

use crate::models::{Categorie, SongData, Vers};

#[derive(Serialize, Deserialize)]
struct Data {
    pub title: String,
    pub category: Categorie,
    pub interpreter: Option<String>,
    pub verses: Vec<String>,
}

#[warn(dead_code)]
pub fn list_content() -> Result<(), Box<dyn std::error::Error>> {
    let raw = fs::read_to_string("songs.json")?;
    dotenv().ok();
    let d = fs::read_to_string("exaltacion.json")?;
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let list_song: Vec<SongData> = serde_json::from_str(&raw)?;
    let d_verses: Vec<Data> = serde_json::from_str(&d)?;

    use crate::schema::songs::dsl::*;
    use crate::schema::verses::dsl::*;
    let mut i = 1;
    let mut j = 1;
    for song in list_song {
        insert_into(songs).values(&song).execute(&mut connection)?;

        let s = d_verses.iter().find(|x| x.title == song.title).unwrap();
        println!("title: {}", s.title);
        for v in s.verses.clone() {
            let inserted = insert_into(verses)
                .values(&Vers {
                    id: j,
                    content: v,
                    song_id: i,
                })
                .execute(&mut connection)?;

            println!("inserted: {}", inserted);
            j += 1;
        }
        i += 1;
    }

    Ok(())
}
