table! {
    auth (user_id) {
        user_id -> Int4,
        login -> Varchar,
        hash_password -> Varchar,
    }
}

table! {
    max_product_id (user_id) {
        user_id -> Int4,
        max_value_id -> Int8,
    }
}

table! {
    products (value_id) {
        value_id -> Int8,
        user_id -> Int4,
        product_id_on_device -> Int8,
        left_node_id -> Int8,
        right_node_id -> Int8,
        image_url -> Text,
        product_title -> Text,
        product_subtitle -> Text,
        expiration_date -> Int8,
        start_tracking_date -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    auth,
    max_product_id,
    products,
);
