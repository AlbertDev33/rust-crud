use diesel::table;

table! {
    users (id) {
        id -> Uuid,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}