use diesel::prelude::*;
use serde::Serialize;
use crate::db;


#[derive(Debug, Queryable, Serialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
}

impl Item {
    pub fn all(conn: &mut db::DbConnection) -> QueryResult<Vec<Item>> {
        use crate::schema::items::dsl::*;

        items.load(conn)
    }
}