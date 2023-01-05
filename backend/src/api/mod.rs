use actix_web::web;

mod users;
mod posts;
mod path;
mod auth;
pub mod util;
mod links;


pub fn api_factory(app: &mut web::ServiceConfig) {
    auth::auth_factory(app);
    users::user_factory(app);
    posts::post_factory(app);
    links::link_factory(app);
}

