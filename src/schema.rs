// @generated automatically by Diesel CLI.

diesel::table! {
    rustaceans (id) {
        id -> Integer,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}
