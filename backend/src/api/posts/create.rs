use actix_web::{web, HttpResponse, HttpRequest};
use crate::database::establish_connection;
use crate::model::post::new_post::NewPost;
use crate::model::user::user::User;
use crate::structs::post::NewPostSchema;

type Model = NewPost;
type JsonSchema = NewPostSchema;

pub async fn create(post: web::Json<JsonSchema>, req: HttpRequest) -> HttpResponse {
    let connection = &mut establish_connection();

    let user = User::find_by_token(&req, connection);

    if let Some(user) = user {
        let new_post = Model::new(post, user);
        let insert_result = Model::create_post(new_post, connection);

        match insert_result {
            Ok(_) => HttpResponse::Created().await.unwrap(),
            Err(_) => HttpResponse::InternalServerError().await.unwrap()
        }
    } else {
        HttpResponse::Unauthorized().await.unwrap()
    }
}