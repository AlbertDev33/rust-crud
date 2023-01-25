use diesel::prelude::*;
use diesel::update;
use futures::future;
use uuid::Uuid;

use crate::database::schemas::users_table::users;
use crate::database::{connection, schemas::users_table::users::dsl::*};

pub async fn delete_user(user_id: Uuid) {
    let db_connection = &mut connection::db_connection().await;
    let deleted_user = chrono::Local::now().naive_local();
    
    future::lazy(|_| {
        update(users::table)
            .filter(id.eq(user_id))
            .set(deleted_at.eq(deleted_user))
            .execute(db_connection)
            .expect("Delete error");
    })
    .await;
}
