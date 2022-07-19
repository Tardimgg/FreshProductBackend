use diesel::prelude::*;
use diesel::pg::PgConnection;
use log::info;

use std::env;
use diesel::connection::SimpleConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn start_db() {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));


    if let Err(err) = conn.batch_execute("CREATE TABLE auth (
    user_id serial PRIMARY KEY,
    login VARCHAR ( 50 ) UNIQUE NOT NULL,
    hash_password VARCHAR ( 255 ) NOT NULL
    );") {
        info!("info message: {}", err);
    }

    if let Err(err) = conn.batch_execute("CREATE TABLE products (
    value_id serial PRIMARY KEY,
    user_id INT NOT NULL,
    product_id_on_device INT NOT NULL,
    image_url TEXT NOT NULL,
    product_title VARCHAR ( 50 ) NOT NULL,
    product_subtitle VARCHAR ( 50 ) NOT NULL,
    expiration_date BIGINT NOT NULL,
    start_tracking_date BIGINT NOT NULL
    );") {
        info!("info message: {}", err);
    }
}

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    // PgConnection::establish(&database_url)
    //     .expect(&format!("Error connecting to {}", database_url));

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}