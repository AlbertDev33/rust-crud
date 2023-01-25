use diesel::prelude::*;
use futures::future;
use uuid::Uuid;

use crate::database::{connection, models::user::User, schemas::users_table::users::dsl::*};

pub async fn get_user_by_id(user_id: Uuid) -> User {
    let db_connection = &mut connection::db_connection().await;
    
    let user = future::lazy(|_| {
        let user: User = users
            .filter(id.eq(user_id))
            .filter(deleted_at.is_null())
            .select((id, first_name, last_name, email, created_at, updated_at))
            .first(db_connection)
            .unwrap_or_else(|err| panic!("User not found {err:?}"));
            
        return user;
    })
    .await;
    return user;
}
