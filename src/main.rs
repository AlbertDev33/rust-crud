#[macro_use] 
extern crate diesel;

mod handlers;
mod database;

use std::env::set_var;
use std::io::Result;

use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> Result<()> {
    set_var("RUST_LOG", "actix_web=debug");
    HttpServer::new(move || {
        App::new()
            .route("/users", web::get().to(handlers::get_users))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::add_user))
            .route("/users/{id}", web::delete().to(handlers::delete_user))
    })
    .bind("127.0.0.1:3333")?
    .run()
    .await
}
