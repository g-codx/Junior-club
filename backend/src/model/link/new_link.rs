use diesel::prelude::*;
use crate::schema::links;
use serde::{Deserialize};

#[derive(Insertable, Deserialize, Clone, Debug)]
#[table_name="links"]
pub struct NewLink {
    pub title: String,
    pub link: String,
    pub link_type: String,
    pub section_type: String
}

impl NewLink {
    pub fn create_link(new_link: NewLink, connection: &mut PgConnection) -> QueryResult<usize> {
        diesel::insert_into(links::table)
            .values(&new_link)
            .execute(connection)
    }
}