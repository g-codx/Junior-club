use actix_web::{HttpResponse, web};
use diesel::prelude::*;

use crate::schema::links::dsl::*;
use crate::schema::links;
use serde::Serialize;
use crate::model::link::json::EditQuery;

#[derive(Queryable, Identifiable, Serialize, Debug)]
#[table_name = "links"]
pub struct LinkInfo {
    pub id: i32,
    pub title: String,
    pub link: String,
    pub link_type: String,
    pub section_type: String
}

impl LinkInfo {
    pub fn find_by_section(section_name: String, connection: &mut PgConnection) -> Vec<LinkInfo> {
        links
            .filter(section_type.eq(section_name))
            .load::<LinkInfo>(connection)
            .unwrap()
    }

    pub fn edit(edit_req: web::Json<EditQuery>, connection: &mut PgConnection) -> HttpResponse {
        let update_result = diesel::update(links.filter(id.eq(edit_req.id)))
            .set((
                title.eq(edit_req.title.clone()),
                link.eq(edit_req.link.clone())
            ))
            .execute(connection);

        match update_result {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => {
                log::info!("{}", e);
                HttpResponse::NotModified().finish()
            }
        }
    }

    pub fn delete(link_id: i32, connection: &mut PgConnection) -> HttpResponse {
        let query_result = diesel::delete(links.filter(id.eq(link_id)))
            .execute(connection);

        match query_result {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => {
                log::info!("{}", e);
                HttpResponse::NotModified().body(e.to_string())
            }
        }
    }
}