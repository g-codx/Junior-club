use actix_web::{HttpRequest, HttpResponse, web};
use crate::api::links::utils::can_execute;
use crate::database::establish_connection;

use crate::model::link::new_link::NewLink;

pub async fn create(link: web::Json<NewLink>, req: HttpRequest) -> HttpResponse {
    let connection = &mut establish_connection();

    if can_execute(&req, connection) {
        let insert_result =  NewLink::create_link(link.0, connection);

        match insert_result {
            Ok(_) => HttpResponse::Created().await.unwrap(),
            Err(err) => {
                log::info!("{}", err);
                HttpResponse::InternalServerError().await.unwrap()
            }
        }
    } else {
        HttpResponse::Forbidden().finish()
    }


}
