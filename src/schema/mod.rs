use diesel::table;

table! {
    auth (user_id) {
        user_id -> Int4,
        login -> Varchar,
        hash_password -> Varchar,
    }
}

table! {
    products (value_id) {
        value_id -> Int4,
        user_id -> Int4,
        product_id_on_device -> Int4,
        image_url -> Text,
        product_title -> Varchar,
        product_subtitle -> Varchar,
        expiration_date -> Int8,
        start_tracking_date -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    auth,
    products,
);
