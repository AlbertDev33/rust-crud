use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use futures::future;
use uuid::Uuid;
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};

use crate::database::models::new_user::{InputUser, NewUser};
use crate::database::models::update_user::{UpdateUser, UpdateUserProps};
use crate::database::operations::delete_user::delete_user;
use crate::database::operations::get_user_by_id::get_user_by_id;
use crate::database::operations::{create_user::create_user, update_user::update_user};
use crate::database::{connection, models::user::User, schemas::users_table::users::dsl::*};

pub async fn get_users() -> HttpResponse {
    let db_connection = &mut connection::db_connection().await;
    let user_list = future::lazy(|_| {
        let user_list: Vec<User> = users
            .select((id, first_name, last_name, email, created_at, updated_at))
            .filter(deleted_at.is_null())
            .load::<User>(db_connection)
            .expect("Error");
        return user_list;
    })
    .await;
    return HttpResponse::Ok().json(user_list);
}

pub async fn get_user_by_id_request(user_id: web::Path<String>) -> HttpResponse {
    let parsed_user_id = Uuid::parse_str(&user_id).unwrap();

    let user = get_user_by_id(parsed_user_id).await;
    return HttpResponse::Ok().json2(&user);
}

pub async fn update_user_request(user_props: web::Json<UpdateUser>) -> HttpResponse {
    let user_props = user_props.into_inner();

    let upuser = UpdateUserProps {
        id: user_props.id,
        first_name: user_props.first_name,
        last_name: user_props.last_name,
        email: user_props.email,
        updated_at: chrono::Local::now().naive_local(),
    };

    let updated_user = update_user(upuser).await;
    return HttpResponse::Ok().json(updated_user);
}

pub async fn create_user_request(user: web::Json<InputUser>) -> HttpResponse {
    let salt = SaltString::generate(&mut OsRng);
    let user_password = user.password.as_bytes();
    let password_hash = Pbkdf2.hash_password(user_password, &salt).expect("Password encrypt error").to_string();

    let new_user = NewUser {
        id: Uuid::new_v4(),
        first_name: String::from(&user.first_name),
        last_name: String::from(&user.last_name),
        email: String::from(&user.email),
        password: password_hash,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };
    let new_user_copy = new_user.clone();
    create_user(new_user).await;
    return HttpResponse::Ok().json(new_user_copy);
}

pub async fn delete_user_request(user_id: web::Path<String>) -> HttpResponse {
    let parse_user_id = Uuid::parse_str(&user_id).expect("Isn't uuid valid");
    delete_user(parse_user_id).await;

    return HttpResponse::Ok().json("Deleted");
}
