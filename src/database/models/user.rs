use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}