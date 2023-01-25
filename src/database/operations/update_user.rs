use futures::future;
use diesel::prelude::*;
use diesel::update;

use crate::database::schemas::users_table::users;
use crate::database::models::update_user::UpdateUserProps;
use crate::database::{connection, models::user::User, schemas::users_table::users::dsl::*};

pub async fn update_user(upuser: UpdateUserProps) -> Vec<User> {
    let db_connection = &mut connection::db_connection().await;
    
    let updated_user = future::lazy(|_| {
        let updated_user: Vec<User> = update(users.filter(id.eq(upuser.id)))
            .set::<&UpdateUserProps>(&upuser)
            .returning((
                users::id,
                users::first_name,
                users::last_name,
                users::email,
                users::created_at,
                users::updated_at,
            ))
            .get_results::<User>(db_connection)
            .unwrap_or_else(|err| panic!("Error connecting to {err:?}"));
        return updated_user;
    })
    .await;
    return updated_user;
}