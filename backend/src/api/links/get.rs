use actix_web::{HttpRequest, Responder, web};
use crate::database::establish_connection;
use crate::model::link::json::{GetQuery, LinksResponse};
use crate::model::user::user::User;

type Model = crate::model::link::link::LinkInfo;


pub async fn get_links(req_query: web::Query<GetQuery>, req: HttpRequest) -> impl Responder {
    let connection = &mut establish_connection();

    let user = User::find_by_token(&req, connection);
    let is_admin = if let Some(user) = user {
        user.role == "admin"
    } else {
        false
    };

    let section_name = req_query.section.clone();
    log::info!("{}", section_name);
    let query_result = Model::find_by_section(section_name, connection);

    LinksResponse::json_response(query_result, is_admin)
}