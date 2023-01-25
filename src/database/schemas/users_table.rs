use diesel::table;

table! {
    users (id) {
        id -> Uuid,
        first_name -> VarChar,
        last_name -> VarChar,
        email -> VarChar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Timestamptz,
    }
}