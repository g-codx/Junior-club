use actix_web::web;
use diesel::{Insertable, PgConnection, RunQueryDsl};
use crate::model::user::user::User;
use crate::schema::posts;
use crate::structs::post::NewPostSchema;
use crate::structs::utils::get_preview;

#[derive(Insertable, Clone, Debug)]
#[table_name="posts"]
pub struct NewPost {
    pub user_id: i32,
    pub title: String,
    pub preview: String,
    pub body: String,
    pub search_tag: String,
}

impl NewPost {

    pub fn new(post: web::Json<NewPostSchema>, user: User) -> Self {
        Self {
            user_id: user.id,
            title: post.title.clone(),
            preview: get_preview(post.body.clone()),
            body: post.body.clone(),
            search_tag: post.search_tag.clone(),
        }
    }

    pub fn create_post(new_post: NewPost, connection: &mut PgConnection) -> diesel::QueryResult<usize> {
        diesel::insert_into(posts::table)
            .values(&new_post)
            .execute(connection)
    }
}

