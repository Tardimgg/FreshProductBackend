-- Your SQL goes here

CREATE TABLE products (
    value_id bigserial PRIMARY KEY,
    user_id INT NOT NULL,
    product_id_on_device BIGINT NOT NULL,
    left_node_id BIGINT NOT NULL,
    right_node_id BIGINT NOT NULL,
    image_url TEXT NOT NULL,
    product_title VARCHAR ( 50 ) NOT NULL,
    product_subtitle VARCHAR ( 50 ) NOT NULL,
    expiration_date BIGINT NOT NULL,
    start_tracking_date BIGINT NOT NULL
);