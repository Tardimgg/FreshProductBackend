-- Your SQL goes here

CREATE TABLE auth (
    user_id serial PRIMARY KEY,
    login VARCHAR ( 50 ) UNIQUE NOT NULL,
    hash_password VARCHAR ( 255 ) NOT NULL
);