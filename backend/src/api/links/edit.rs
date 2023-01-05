use actix_web::{HttpRequest, HttpResponse, web};
use crate::api::links::utils::can_execute;
use crate::database::establish_connection;
use crate::model;
use crate::model::link::json::EditQuery;

type Model = model::link::link::LinkInfo;

pub async fn edit(edit_req: web::Json<EditQuery>, req: HttpRequest) -> HttpResponse {
    let connection = &mut establish_connection();

    if can_execute(&req, connection) {
        Model::edit(edit_req, connection)
    } else {
        HttpResponse::Forbidden().finish()
    }


}