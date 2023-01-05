use actix_web::{HttpResponse, web};
use actix_web::http::header::{AUTHORIZATION, CONTENT_TYPE};
use diesel::prelude::*;
use crate::database::establish_connection;
use crate::model::user::user::User;

use crate::schema::users;
use crate::auth::jwt::JwtToken;
use crate::structs::login::Login;

pub async fn login(credentials: web::Json<Login>) -> HttpResponse {
    let connection = &mut establish_connection();

    let username: String = credentials.username.clone();
    let password: String = credentials.password.clone();

    let users = users::table
        .filter(users::columns::name.eq(username.as_str()))
        .load::<User>(connection).unwrap();

    if users.len() == 0 {
        return HttpResponse::NotFound().await.unwrap()
    } else if users.len() > 1 {
        log::error!("multiple users have the username: {}",credentials.username.clone());
        return HttpResponse::Conflict().await.unwrap()
    }

    match users[0].clone().verify(password) {
        true => {
            let token: String = JwtToken::encode(users[0].clone().id);
            log::info!("{}", token);

            HttpResponse::Ok()
                .append_header((AUTHORIZATION, token.clone()))
                .append_header((CONTENT_TYPE, "application/json"))
                .await
                .unwrap()
        },
        false => HttpResponse::Unauthorized().await.unwrap()
    }
}