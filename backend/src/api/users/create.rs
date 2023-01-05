use actix_web::{HttpResponse, web};
use diesel::prelude::*;
use crate::database::establish_connection;
use crate::model::user::new_user::NewUser;
use crate::structs::user::NewUserSchema;

type Model = NewUser;
type JsonSchema = NewUserSchema;

//todo There is validation on the frontend, but you can directly create a user with any parameters.
// To be fixed
pub async fn create(new_user_schema: web::Json<JsonSchema>) -> HttpResponse {
    use crate::schema::users;
    let connection = &mut establish_connection();

    let new_user = Model::new(new_user_schema);

    let insert_result = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection);

    log::info!("{:?}", insert_result);

    match insert_result {
        Ok(_) => HttpResponse::Created().await.unwrap(),
        Err(_) => HttpResponse::Conflict().await.unwrap()
    }
}