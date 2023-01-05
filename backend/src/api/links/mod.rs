use actix_web::web;

use crate::api::path::Path;

mod create;
mod delete;
mod edit;
mod get;
mod utils;


///Specific section API (Java, Rust tabs)
pub fn link_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path { prefix : String::from("/link") };

    app.route(
        &base_path.define(String::from("/get")),
        web::get().to(get::get_links)
    );

    app.route(
        &base_path.define(String::from("/create")),
        web::post().to(create::create)
    );

    app.route(
        &base_path.define(String::from("/edit")),
        web::put().to(edit::edit)
    );

    app.route(
        &base_path.define(String::from("/delete/{id}")),
        web::delete().to(delete::delete)
    );


}