use std::fmt;

use crate::schema::{songs, sql_types::Category};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Clone, Debug, Serialize, Deserialize)]
#[diesel(table_name = songs)]
pub struct Song {
    pub id: i32,
    pub name: String,
    pub category: Category,
    pub interpreter: String,
    pub num_verses: i32,
}

impl fmt::Display for Song {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}, name: {}, category: {:?}, interpreter: {}, num_verses: {}",
            self.id, self.name, self.category, self.interpreter, self.num_verses
        )
    }
}

// #[derive(Queryable, Selectable, AsChangeset, Serialize, Deserialize)]
// #[diesel(table_name = books)]
// pub struct BookData {
//     pub title: String,
//     pub description: String,
//     pub author: String,
//     pub published: bool,
// }
//
// #[derive(Insertable)]
// #[diesel(table_name = books)]
// pub struct NewBook<'a> {
//     pub title: &'a str,
//     pub description: &'a str,
//     pub author: &'a str,
//     pub published: bool,
// }
