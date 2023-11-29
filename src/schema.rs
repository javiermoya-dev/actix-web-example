// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
    }
}
