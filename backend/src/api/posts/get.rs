use actix_web::{web, Responder, HttpRequest};
use crate::database::establish_connection;
use crate::api::util::parse_id_from_param;

use crate::model::post::post_criteria::{Criteria, Query};
use crate::model::user::user::User;
use crate::structs::post::{GetQuery, Posts};

type Model = crate::model::post::post::Post;
type JsonSchema = crate::structs::post::Post;


pub async fn get_posts(req_query: web::Query<GetQuery>, req: HttpRequest) -> impl Responder {
    let connection = &mut establish_connection();

    let user = User::find_by_token(&req, connection);
    let criteria = Criteria::from(req_query, user);

    let result = match criteria.query {
        Query::All => Model::find_all(criteria.page, connection),
        Query::AllByTag => Model::find_all_by_tag(
            criteria.page, criteria.tag.unwrap().get_tag(), connection),
        Query::Published => Model::find_published(criteria.page, connection),
        Query::PublishedByTag => Model::find_published_by_tag(
            criteria.page, criteria.tag.unwrap().get_tag(), connection),
        Query::UserQuery => Model::find_published_or_user_created(
            criteria.page, criteria.user_id.unwrap(), connection),
        Query::UserQueryByTag => Model::find_published_or_user_created_by_tag(
            criteria.page, criteria.user_id.unwrap(),
            criteria.tag.unwrap().get_tag(), connection)
    };

    Posts::new(result.0, result.1)
}


pub async fn get_post(req: HttpRequest) -> impl Responder {
    let connection = &mut establish_connection();

    match parse_id_from_param(&req) {
        Ok(post_id) => {
            let user = User::find_by_token(&req, connection);
            let result = Model::find_by_id(post_id, connection);
            JsonSchema::new_post(result, user)
        }
        Err(err) => JsonSchema::empty(err.to_string())
    }
}

