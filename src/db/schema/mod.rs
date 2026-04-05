diesel::table! {
    users (id) {
        id         -> Int4,
        username   -> Varchar,
        email      -> Varchar,
        full_name  -> Nullable<Varchar>,
        active     -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
