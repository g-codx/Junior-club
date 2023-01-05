use actix_web::{HttpResponse, HttpRequest};
use diesel::RunQueryDsl;
use crate::database::establish_connection;
use diesel::prelude::*;
use crate::api::posts::utils::can_execute;
use crate::api::util::parse_id_from_param;

pub async fn delete(req: HttpRequest) -> HttpResponse {
    use crate::schema::posts::dsl::*;
    let connection = &mut establish_connection();


    match parse_id_from_param(&req) {
        Ok(post_id) => {
            if can_execute(&req, connection, post_id) {

                let query_result = diesel::delete(posts.filter(id.eq(post_id)))
                    .execute(connection);

                match query_result {
                    Ok(_) => HttpResponse::Ok().finish(),
                    Err(e) => {
                        log::info!("{}", e);
                        HttpResponse::NotModified().body(e.to_string())
                    }
                }

            } else {
                HttpResponse::Forbidden().finish()
            }
        },
        Err(err) => HttpResponse::BadRequest().body(err.to_string())
    }
}

