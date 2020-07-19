table! {
    projects (id) {
        id -> Int4,
        name -> Varchar,
        info -> Text,
        priority -> Int4,
    }
}

table! {
    trackings (username, project_id, created_time) {
        username -> Varchar,
        project_id -> Int4,
        created_time -> Timestamp,
        recorded_time -> Float4,
    }
}

table! {
    users (username) {
        username -> Varchar,
        password -> Varchar,
        display_name -> Varchar,
    }
}

joinable!(trackings -> projects (project_id));
joinable!(trackings -> users (username));

allow_tables_to_appear_in_same_query!(
    projects,
    trackings,
    users,
);
