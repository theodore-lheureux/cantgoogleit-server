// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Nullable<BigInt>,
        description -> Text,
        installed_on -> Timestamp,
        success -> Bool,
        checksum -> Binary,
        execution_time -> BigInt,
    }
}

diesel::table! {
    classrooms (id) {
        id -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    positions (id) {
        id -> Nullable<Integer>,
        owner_id -> Integer,
        classroom_id -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::joinable!(positions -> classrooms (classroom_id));
diesel::joinable!(positions -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    classrooms,
    positions,
    users,
);
