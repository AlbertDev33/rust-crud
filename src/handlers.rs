use actix_web::Responder;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub async fn get_users() -> impl Responder {
    format!("Hello from get users")
}

pub async fn get_user_by_id() -> impl Responder {
    format!("Hello from get users by id")
}

pub async fn add_user() -> impl Responder {
    format!("Hello from add user")
}

pub async fn delete_user() -> impl Responder {
    format!("Hello from delete user")
}