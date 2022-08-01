use actix_web::{get, post, delete, HttpResponse, Responder, web};
use actix_web::http::StatusCode;
use diesel::{EqAll, QueryDsl, RunQueryDsl};
use crate::data_base::DbPool;
use crate::models::{CroppedProduct, NewProduct, Product, JsonResponse, User};

use serde::Deserialize;
use crate::auth_api::{check_user_registration, get_auth_user};
use crate::logo_api::get_logo_url;
use crate::schema::products;

#[derive(Deserialize)]
pub struct RequestCreateNewProduct {
    login: String,
    password: String,
    product_id_on_device: i32,
    image_url: String,
    product_title: String,
    product_subtitle: String,
    expiration_date: i64,
    start_tracking_date: i64
}

#[post("/product")]
pub async fn add_product(db_pool: web::Data<DbPool>, request: web::Json<RequestCreateNewProduct>) -> impl Responder {

    let conn = db_pool.get().unwrap();
    let request = request.into_inner();

    match get_auth_user(check_user_registration(&db_pool, request.login, &request.password).await) {
        Ok(user) => {
            let mut new_product = NewProduct {
                user_id: user.user_id,
                product_id_on_device: request.product_id_on_device,
                image_url: request.image_url,
                product_title: request.product_title,
                product_subtitle: request.product_subtitle,
                expiration_date: request.expiration_date,
                start_tracking_date: request.start_tracking_date,
            };

            if new_product.image_url.trim() == "" {
                new_product.image_url = match get_logo_url(&new_product.product_subtitle).await {
                    Ok(mut v) => { if v.len() > 0 { v.swap_remove(0)} else { "".to_string() } }
                    Err(_) => { "".to_string() }
                }
            }

            match web::block(move || {
                products::dsl::products
                    .filter(products::user_id.eq_all(new_product.user_id))
                    .filter(products::product_id_on_device.eq_all(new_product.product_id_on_device))
                    .limit(1)
                    .load::<Product>(&*conn)
            }).await {
                Ok(v) => {
                    match v {
                        Ok(res) => {
                            if res.len() != 0 {
                                return HttpResponse::BadRequest().body(String::from("this entry already exists"));
                            }
                        }
                        Err(v) => { return HttpResponse::BadRequest().body(v.to_string()); }
                    }
                }
                Err(_) => { return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish(); }
            }

            let conn = db_pool.get().unwrap();

            match web::block(move || {
                diesel::insert_into(products::table)
                    .values(new_product)
                    .execute(&*conn)

            }).await {
                Ok(v) => {
                    match v {
                        Ok(_) => { HttpResponse::Ok().body("success") }
                        Err(v) => { HttpResponse::BadRequest().body(v.to_string()) }
                    }}
                Err(_) => { HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish() }
            }
        }
        Err(err) => { err }
    }
}

#[post("/get_products")]
pub async fn get_products(db_pool: web::Data<DbPool>, user: web::Json<User>) -> impl Responder {

    let conn = db_pool.get().unwrap();
    let user = user.into_inner();


    return match get_auth_user(check_user_registration(&db_pool, user.login, &user.password).await) {
        Ok(auth_user) => {
            match web::block(move || {
                products::dsl::products
                    .filter(products::user_id.eq_all(auth_user.user_id))
                    .load::<Product>(&*conn)
            }).await {
                Ok(v) => {
                    match v {
                        Ok(res) => {
                            HttpResponse::Ok().json(JsonResponse::new(res.into_iter().map(|v| v.cutting()).collect::<Vec<CroppedProduct>>()))
                        }
                        Err(v) => { HttpResponse::Ok().json(JsonResponse::new(v.to_string())) }
                    }
                }
                Err(_) => { HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish() }
            }
        }
        Err(err) => { err }
    };
}

#[derive(Deserialize)]
pub struct RequestDeleteProduct {
    pub login: String,
    pub password: String,
    pub product_id_on_device: i32
}

#[delete("/product")]
pub async fn delete_product(db_pool: web::Data<DbPool>, request: web::Json<RequestDeleteProduct>) -> impl Responder {

    let conn = db_pool.get().unwrap();
    let request = request.into_inner();

    return match get_auth_user(check_user_registration(&db_pool, request.login, &request.password).await) {
        Ok(auth_user) => {
            match web::block(move || {

                diesel::delete(
                    products::dsl::products
                        .filter(products::user_id.eq_all(auth_user.user_id))
                        .filter(products::product_id_on_device.eq_all(request.product_id_on_device))
                ).execute(&*conn)

            }).await {
                Ok(v) => {
                    match v {
                        Ok(_) => { HttpResponse::Ok().body("success") }
                        Err(v) => { HttpResponse::BadRequest().body(v.to_string()) }
                    }
                }
                Err(_) => { HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish() }
            }
        }
        Err(err) => { err }
    };
}