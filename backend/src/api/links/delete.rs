use actix_web::{HttpRequest, HttpResponse};
use crate::api::links::utils::can_execute;
use crate::api::util::parse_id_from_param;
use crate::database::establish_connection;
use crate::model::link::link::LinkInfo;

type Model = LinkInfo;

pub async fn delete(req: HttpRequest) -> HttpResponse {
    let connection = &mut establish_connection();

    if can_execute(&req, connection) {
        match parse_id_from_param(&req) {
            Ok(link_id) => {
                log::info!("{}", link_id);
                Model::delete(link_id, connection)
            },
            Err(err) => {
                HttpResponse::BadRequest().body(err.to_string())
            }
        }
    } else {
        HttpResponse::Forbidden().finish()
    }


}