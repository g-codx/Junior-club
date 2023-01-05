use actix_web::HttpRequest;
use diesel::PgConnection;
use crate::model::user::user::User;

pub fn can_execute(req: &HttpRequest, connection: &mut PgConnection) -> bool {
    let user = User::find_by_token(req, connection);

    if let Some(user) = user {
        println!("{:?}", user);
        user.role == "admin"
    } else {
        println!("user not found");
        false
    }
}