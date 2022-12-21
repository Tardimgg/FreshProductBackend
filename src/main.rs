#[macro_use]
extern crate diesel;

mod receipts_api;
mod data_base;
mod models;
mod schema;
mod auth_api;
mod products_api;
mod hashing_api;
mod logo_api;

use dotenv::dotenv;

use std::env;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use crate::receipts_api::get_receipt_info;
use auth_api::register;
use crate::auth_api::check_registration;
use crate::data_base::{get_connection_pool, start_db};
use crate::products_api::{add_product, delete_product, get_products, update_neighbors, update_product};
use crate::logo_api::find_logo;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    if env::var("HEROKU").is_err() {
        dotenv().ok();
    }

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    start_db();
    let pool = Data::new(get_connection_pool());


    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(hello)
            .service(get_receipt_info)
            .service(register)
            .service(add_product)
            .service(get_products)
            .service(delete_product)
            .service(find_logo)
            .service(check_registration)
            .service(update_product)
            .service(update_neighbors)
            // .route("/hey", web::get().to(manual_hello))
    })
        .bind(("0.0.0.0", port))
        .expect("Can not bind to port")
        .run()
        .await
}

