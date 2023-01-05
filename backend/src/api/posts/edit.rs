use actix_web::{web, HttpResponse, HttpRequest};
use chrono::Utc;
use crate::database::establish_connection;
use crate::structs::post::{EditQuery, PublishQuery};

use diesel::prelude::*;
use crate::api::posts::utils::can_execute;
use crate::structs::utils::get_preview;

//todo Если передать несуществующий id вернет 200.
//todo Перед апдейтом нужно проверить если ли запись
pub async fn edit(req: HttpRequest, edit_req: web::Json<EditQuery>) -> HttpResponse {
    use crate::schema::posts::dsl::*;
    let connection = &mut establish_connection();

    let post_id = edit_req.id;

    if can_execute(&req, connection, post_id) {
        let post_title = edit_req.title.clone();
        let post_body = edit_req.body.clone();

        let post_preview = get_preview(post_body.clone());

        let update_row = diesel::update(posts.filter(id.eq(post_id)))
            .set((title.eq(post_title), preview.eq(post_preview), body.eq(post_body)))
            .execute(connection);

        match update_row {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => {
                log::info!("{:?}", e);
                HttpResponse::NotModified().finish()
            }
        }
    } else {
        HttpResponse::Forbidden().finish()
    }


}

pub async fn publish(req: HttpRequest, post: web::Json<PublishQuery>) -> HttpResponse {
    use crate::schema::posts::dsl::*;
    let connection = &mut establish_connection();

    let post_id = post.id;

    if can_execute(&req, connection, post_id) {
        let (publish, naive_date_time) = if post.publish {
            (true, Some(Utc::now().naive_utc()))
        } else {
            (false, None)
        };

        let update_row = diesel::update(posts.filter(id.eq(post_id)))
            .set((published.eq(publish),(public_date.eq(naive_date_time))))
            .execute(connection);

        match update_row {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => {
                log::info!("{:?}", e);
                HttpResponse::NotModified().finish()
            }
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }




}

