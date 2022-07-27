-- Your SQL goes here

CREATE TABLE products (
    value_id serial PRIMARY KEY,
    user_id INT NOT NULL,
    product_id_on_device INT NOT NULL,
    image_url TEXT NOT NULL,
    product_title VARCHAR ( 50 ) NOT NULL,
    product_subtitle VARCHAR ( 50 ) NOT NULL,
    expiration_date BIGINT NOT NULL,
    start_tracking_date BIGINT NOT NULL
);