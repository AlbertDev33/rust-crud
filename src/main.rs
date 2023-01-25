#[macro_use]
extern crate diesel;

mod database;
mod handlers;

use std::env::set_var;
use std::io::Result;

use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> Result<()> {
    set_var("RUST_LOG", "actix_web=debug");

    return HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/users")
                    .route(web::get().to(handlers::get_users))
                    .route(web::post().to(handlers::create_user_request))
                    .route(web::put().to(handlers::update_user_request)),
            )
            .service(
                web::resource("/users/{id}")
                    .route(web::get().to(handlers::get_user_by_id_request))
                    .route(web::delete().to(handlers::delete_user_request)),
            )
    })
    .bind("127.0.0.1:3333")?
    .run()
    .await;
}
