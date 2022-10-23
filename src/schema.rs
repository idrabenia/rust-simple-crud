// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        active -> Bool,
        sign_in_count -> Int8,
    }
}
