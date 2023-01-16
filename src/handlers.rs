use actix_web::{web, Responder};
use diesel::insert_into;
use diesel::prelude::*;
use futures::future;
use serde::{Deserialize, Serialize};
use std::{thread, time::Instant};
use uuid::Uuid;

use crate::database::models::new_user::NewUser;
use crate::database::schemas::users::users;
use crate::database::{connection, models::user::User, schemas::users::users::dsl::*};
#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub async fn get_users() -> web::Json<Vec<User>> {
    let db_connection = connection::db_connection();
    let user_list = users
        .select((id, first_name, last_name, email, created_at, updated_at))
        .load(&db_connection)
        .expect("Error");
    return web::Json(user_list);
}

pub async fn get_user_by_id() -> impl Responder {
    format!("Hello from get users by id")
}

// pub async fn add_user(user: web::Json<InputUser>) -> web::Json<NewUser> {
//     let start = Instant::now();
//     let db_connection = connection::db_connection();

//     let new_user = NewUser {
//         id: Uuid::new_v4(),
//         first_name: String::from(&user.first_name),
//         last_name: String::from(&user.last_name),
//         email: String::from(&user.email),
//         created_at: chrono::Local::now().naive_local(),
//         updated_at: chrono::Local::now().naive_local(),
//     };
//     let new_user_copy = new_user.clone();
//     println!("Inicio da thread");
//     thread::spawn(move || {
//         insert_into(users::table)
//             .values(&new_user)
//             .execute(&db_connection)
//             .unwrap();
//         println!("Dentro da thread");
//     });
//     let duration = start.elapsed();
//     println!("Fim da thread");
//     println!("Time {duration:?}");
//     return web::Json(new_user_copy);
// }

pub async fn add_user(user: web::Json<InputUser>) -> web::Json<NewUser> {
    let start = Instant::now();
    let db_connection = connection::db_connection();

    let new_user = NewUser {
        id: Uuid::new_v4(),
        first_name: String::from(&user.first_name),
        last_name: String::from(&user.last_name),
        email: String::from(&user.email),
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };
    let new_user_copy = new_user.clone();
    println!("Inicio da future");
    future::lazy(|cx| {
        insert_into(users::table)
            .values(&new_user)
            .execute(&db_connection)
            .unwrap();
        println!("Dentro da future");
    })
    .await;
    println!("Fim da future");
    // thread::spawn(move || {

    // });
    let duration = start.elapsed();
    println!("Time {duration:?}");
    return web::Json(new_user_copy);
}

// pub async fn add_user(user: web::Json<InputUser>) -> web::Json<User> {
//     let start = Instant::now();
//     let db_connection = connection::db_connection();
//     let new_user = NewUser {
//         id: Uuid::new_v4(),
//         first_name: String::from(&user.first_name),
//         last_name: String::from(&user.last_name),
//         email: String::from(&user.email),
//         created_at: chrono::Local::now().naive_local(),
//         updated_at: chrono::Local::now().naive_local(),
//     };
//     let user = insert_into(users::table)
//         .values(&new_user)
//         .returning((
//             users::id,
//             users::first_name,
//             users::last_name,
//             users::email,
//             users::created_at,
//             users::updated_at,
//         ))
//         .get_result(&db_connection)
//         .unwrap_or_else(|_| panic!("Error connecting to"));

//     let duration = start.elapsed();
//     println!("Time {duration:?}");
//     return web::Json(user);
// }

pub async fn delete_user() -> impl Responder {
    format!("Hello from delete user")
}
