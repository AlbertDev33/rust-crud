use chrono::{DateTime, Utc};
use diesel::Queryable;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}