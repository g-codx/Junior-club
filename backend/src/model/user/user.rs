use actix_web::HttpRequest;
use bcrypt::verify;
use diesel::prelude::*;
use log::info;
use crate::api::util::parse_user_id_from_header;
use crate::schema::users;
use crate::schema::users::dsl::*;



#[derive(Queryable, Clone, Identifiable, Debug)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
    pub role: String,
}

impl User {

    /// Checks password to see if correct.
    ///
    /// # Arguments
    /// * password (String): the password to be checked against the user password
    ///
    /// # Returns
    /// * (bool): true is match, false if not
    pub fn verify(self, pass: String) -> bool {
        return verify(pass.as_str(), &self.password).unwrap()
    }

    pub fn find_by_id(user_id: i32, connection: &mut PgConnection) -> Option<User> {
        match users.filter(id.eq(user_id)).load::<User>(connection) {
            Ok(user) => {
                user.first().cloned()
            },
            Err(e) => {
                info!("{}", e);
                None
            }
        }
    }

    pub fn find_by_token(req: &HttpRequest, connection: &mut PgConnection) -> Option<User> {
        let some_user_id = parse_user_id_from_header(req);
        match some_user_id {
            Some(usr_id) => {
                User::find_by_id(usr_id, connection)
            },
            None => {
                info!("invalid parameter id");
                None
            }
        }
    }
}
