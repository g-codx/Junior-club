use actix_web::HttpRequest;
use diesel::PgConnection;
use crate::model::post::post::Post;
use crate::model::user::user::User;

pub fn can_execute(req: &HttpRequest, connection: &mut PgConnection, post_id: i32) -> bool {
    let user = User::find_by_token(req, connection);
    println!("{:?}", user);

    return if let Some(user) = user {
        if user.role == "admin" {
            return true;
        }

        let post = Post::find_by_id(post_id, connection);
        log::info!("post info - {:?}", post);

        return  post.1 == user.id
    } else {
        false
    }
}