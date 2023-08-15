use std::fmt;

use crate::schema::songs;
use crate::schema::verses;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::Category"]
pub enum Categorie {
    #[serde(alias = "exaltacion")]
    Exaltacion,
    #[serde(alias = "adoracion")]
    Adoracion,
    #[serde(alias = "ministracion")]
    Ministracion
}

#[derive(Queryable, Selectable, Clone, Debug, Serialize, Deserialize)]
#[diesel(table_name = songs)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub category: Categorie,
    pub interpreter: Option<String>,
    pub num_verses: i32,
}

#[derive(Insertable, Queryable, Selectable, Clone, Debug, Serialize, Deserialize)]
#[diesel(table_name = verses)]
pub struct Vers {
    pub id: i32,
    pub content: String,
    pub song_id: i32,
}

impl fmt::Display for Song {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}, name: {}, category: {:?}, interpreter: {:?}, num_verses: {}",
            self.id, self.title, self.category, self.interpreter, self.num_verses
        )
    }
}


#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = songs)]
pub struct SongData {
    pub title: String,
    pub category: Categorie,
    pub interpreter: Option<String>,
    pub num_verses: i32,
}
//
// #[derive(Insertable)]
// #[diesel(table_name = books)]
// pub struct NewBook<'a> {
//     pub title: &'a str,
//     pub description: &'a str,
//     pub author: &'a str,
//     pub published: bool,
// }
