use diesel::insert_into;
use diesel::prelude::*;
use futures::future;
// use std::thread;

use crate::database::connection;
use crate::database::models::new_user::NewUser;
use crate::database::schemas::users_table::users;

pub async fn create_user(new_user: NewUser) {
    let db_connection = &mut connection::db_connection().await;

    future::lazy(|_| {
        insert_into(users::table)
            .values(&new_user)
            .execute(db_connection)
            .expect("User insertion error");
    })
    .await;
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
