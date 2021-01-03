table! {
    notes (id) {
        id -> Int4,
        uuid -> Varchar,
        title -> Varchar,
        description -> Text,
        create_time -> Varchar,
        last_update_time -> Varchar,
        delete_time -> Nullable<Varchar>,
    }
}
