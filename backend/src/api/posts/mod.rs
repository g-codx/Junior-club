use actix_web::web;
use crate::api::path::Path;

mod create;
mod edit;
mod delete;
mod get;
mod utils;


///Post API
pub fn post_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path { prefix: String::from("/post") };

    // Create new post with status non published
    app.route(
        &base_path.define(String::from("/create")),
        web::post().to(create::create)
    );

    // Get feed post by search tag (published, non published optionally)
    app.route(
        &base_path.define(String::from("/get")),
        web::get().to(get::get_posts)
    );

    // Get card post by id (published, non published)
    app.route(
        &base_path.define(String::from("/get/{id}")),
        web::get().to(get::get_post)
    );

    // Update post (title, preview, body)
    app.route(
        &base_path.define(String::from("/edit")),
        web::put().to(edit::edit)
    );

    // Edit publish status (publish true - passed moderation)
    app.route(
        &base_path.define(String::from("/publish")),
        web::put().to(edit::publish)
    );

    // Delete post by id
    app.route(
        &base_path.define(String::from("/delete/{id}")),
        web::delete().to(delete::delete)
    );
}

