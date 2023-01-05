use actix_web::web;
use bcrypt::{DEFAULT_COST, hash};
use diesel::prelude::*;
use uuid::Uuid;


use crate::schema::users;
use crate::structs::user::NewUserSchema;

#[derive(Insertable, Clone)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
    pub role: String,
}

//todo возможно убрать email. нафига он ?
impl NewUser {
    pub fn new(user: web::Json<NewUserSchema>) -> Self {
        let hashed_password: String = hash(user.password.as_str(),DEFAULT_COST).unwrap();
        let uuid = Uuid::new_v4();
        Self {
            name: user.name.clone(),
            email: user.email.clone(),
            password: hashed_password,
            unique_id: uuid.to_string(),
            role: "user".to_string()
        }
    }
}