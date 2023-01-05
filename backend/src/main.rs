use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer};
use actix_web::dev::Service;

use log;
use env_logger;
use futures::future::{Either, ok};
use crate::auth::gateway::passed;

mod api;
mod model;
mod database;
mod schema;
mod structs;
mod auth;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap_fn(|mut req, srv| {
                let request_url: String = String::from(*&req.uri().path().clone());

                let passed = passed(&mut req);

                let end_result = match passed {
                    true => {
                        Either::Left(srv.call(req))
                    },
                    false => {
                        Either::Right(
                            ok(req.into_response(HttpResponse::Unauthorized().body("")))
                        )
                    }
                };

                async move {
                    let result = end_result.await?;
                    log::info!("{} -> {}", request_url, &result.status());
                    Ok(result)
                }

            })
            .wrap(
                // Cors::default()
                //     .allowed_origin("http://localhost:3000")
                //     .allowed_origin_fn(|origin, _req_head| { origin.as_bytes().ends_with(b".localhost:3000") })
                //     .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
                //     .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                //     .allowed_header(header::CONTENT_TYPE)
                //     .supports_credentials()
                //     .allowed_header(ACCESS_CONTROL_ALLOW_CREDENTIALS)
                // .max_age(3600)

                Cors::permissive()

            )
            .configure(api::api_factory)
    })
        .bind(("0.0.0.0", 8000))?
        .workers(12)
        .run()
        .await
}



