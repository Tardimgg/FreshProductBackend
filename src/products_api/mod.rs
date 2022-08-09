use std::collections::HashMap;
use actix_web::{get, post, delete, patch, HttpResponse, Responder, web};
use actix_web::http::StatusCode;
use diesel::{EqAll, QueryDsl, QueryResult, RunQueryDsl};
use crate::data_base::DbPool;
use crate::models::{CroppedProduct, NewProduct, Product, JsonResponse, User, MaxProductId};

use serde::{Serialize, Deserialize};
use crate::auth_api::{check_user_registration, get_auth_user};
use crate::logo_api::get_logo_url;
use crate::schema::products;
use crate::schema::max_product_id;

#[derive(Serialize)]
pub struct ProductIdResponse {
    new_product_id: i64,
}

#[derive(Deserialize)]
pub struct RequestCreateNewProduct {
    login: String,
    password: String,
    product_id_on_device: i64, // must be delete
    left_node_id: i64,
    right_node_id: i64,
    image_url: String,
    product_title: String,
    product_subtitle: String,
    expiration_date: i64,
    start_tracking_date: i64, // must be delete
}

#[post("/product")]
pub async fn add_product(db_pool: web::Data<DbPool>, request: web::Json<RequestCreateNewProduct>) -> impl Responder {
    let request = request.into_inner();

    let user = get_auth_user(check_user_registration(&db_pool, request.login, &request.password).await);

    if let Err(err) = user {
        return err;
    }

    let user = user.unwrap();

    let mut new_product = NewProduct {
        user_id: user.user_id,
        product_id_on_device: request.product_id_on_device,
        left_node_id: request.left_node_id,
        right_node_id: request.right_node_id,
        image_url: request.image_url,
        product_title: request.product_title,
        product_subtitle: request.product_subtitle,
        expiration_date: request.expiration_date,
        start_tracking_date: request.start_tracking_date,
    };

    let new_id = get_max_product_id(&db_pool, user.user_id).await;
    new_product.product_id_on_device = match new_id {
        Ok(v) => { v + 1 }
        Err(e) => { return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e); }
    };
    match push_max_product_id(&db_pool, user.user_id, new_product.product_id_on_device).await {
        Ok(_) => {}
        Err(_) => { return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish(); }
    };

    update_logo(&new_product.product_subtitle, &mut new_product.image_url).await;

    let current_product_id = new_product.product_id_on_device;

    let conn = db_pool.get().unwrap();
    match web::block(move || {
        diesel::insert_into(products::table)
            .values(new_product)
            .execute(&*conn)
    }).await {
        Ok(v) => {
            match v {
                Ok(_) => {
                    let response = ProductIdResponse {
                        new_product_id: current_product_id,
                    };

                    HttpResponse::Ok().json(response) }
                Err(v) => { HttpResponse::BadRequest().body(v.to_string()) }
            }
        }
        Err(_) => { HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish() }
    }
}

#[derive(Deserialize)]
pub struct RequestUpdateNeighbors {
    data: HashMap<i64, Vec<i64>>
}

#[post("/update_neighbors_products")]
pub async fn update_neighbors(db_pool: web::Data<DbPool>, request: web::Json<RequestUpdateNeighbors>) -> impl Responder {
    let request = request.into_inner();
    let conn = db_pool.get().unwrap();

    match web::block(move || {
        for (id, value) in request.data {
            if value.len() != 2 {
                return Err("each element must have 2 neighbors".to_string());
            }

            if let Err(e) = diesel::update(products::table.filter(products::product_id_on_device.eq_all(id)))
                .set((products::left_node_id.eq_all(value[0]), products::right_node_id.eq_all(value[1])))
                // .values(new_product)
                .execute(&*conn) {

                return Err(e.to_string());
            }
        }
        return Ok("success");
    }).await {
        Ok(v) => {
            match v {
                Ok(v) => { HttpResponse::Ok().json(JsonResponse::new(v)) }
                Err(e) => { HttpResponse::BadRequest().json(JsonResponse::new(e)) }
            }
        }
        Err(_) => { HttpResponse::InternalServerError().finish() }
    }
}

async fn get_max_product_id(db_pool: &web::Data<DbPool>, user_id: i32) -> Result<i64, String> {
    let conn = db_pool.get().unwrap();

    let max_id = web::block(move || {
        max_product_id::dsl::max_product_id
            .filter(max_product_id::user_id.eq_all(user_id))
            .select(max_product_id::max_value_id)
            .limit(1)
            .load::<i64>(&*conn)
    }).await;

    match max_id {
        Ok(v) => {
            match v {
                Ok(v) => {
                    if v.len() == 0 {
                        match push_max_product_id(db_pool, user_id, 0).await {
                            Ok(_) => { Ok(0) }
                            Err(e) => { Err(e) }
                        }
                    } else {
                        Ok(v[0])
                    }
                }
                Err(e) => { Err(e.to_string()) }
            }
        }
        Err(_) => { Err(String::from("internal error")) }
    }
}

async fn push_max_product_id(db_pool: &web::Data<DbPool>, user_id: i32, new_max: i64) -> Result<(), String> {
    let conn = db_pool.get().unwrap();

    let new_value = MaxProductId {
        user_id,
        max_value_id: new_max
    };

    let res = web::block(move || {
        diesel::insert_into(max_product_id::table)
            .values(new_value)
            .on_conflict(max_product_id::user_id)
            .do_update()
            .set(new_value)
            .execute(&*conn)
    }).await;

    match res {
        Ok(_) => { Ok(()) }
        Err(_) => { Err(String::from("internal error")) }
    }
}

#[patch("/product")]
pub async fn update_product(db_pool: web::Data<DbPool>, request: web::Json<RequestCreateNewProduct>) -> impl Responder {
    let conn = db_pool.get().unwrap();
    let request = request.into_inner();

    match get_auth_user(check_user_registration(&db_pool, request.login, &request.password).await) {
        Ok(user) => {
            let mut new_product = NewProduct {
                user_id: user.user_id,
                product_id_on_device: request.product_id_on_device,
                left_node_id: request.left_node_id,
                right_node_id: request.right_node_id,
                image_url: request.image_url,
                product_title: request.product_title,
                product_subtitle: request.product_subtitle,
                expiration_date: request.expiration_date,
                start_tracking_date: request.start_tracking_date,
            };

            update_logo(&new_product.product_subtitle, &mut new_product.image_url).await;

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
                            if res.len() != 1 {
                                return HttpResponse::BadRequest().body(String::from("required entry does not exist"));
                            }
                        }
                        Err(v) => { return HttpResponse::BadRequest().body(v.to_string()); }
                    }
                }
                Err(_) => { return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish(); }
            }

            let conn = db_pool.get().unwrap();

            match web::block(move || {
                diesel::update(products::table.filter(products::product_id_on_device.eq_all(new_product.product_id_on_device)))
                    .set(new_product)
                    // .values(new_product)
                    .execute(&*conn)
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
    }
}

async fn update_logo(name: &str, url: &mut String) {
    if url.trim() == "" {
        match get_logo_url(name).await {
            Ok(mut v) => {
                *url = if v.len() > 0 { v.swap_remove(0) } else { "".to_string() };
            }
            Err(_) => {
                url.clear();
            }
        }
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
    pub product_id_on_device: i64
}

#[post("/delete_product")]
pub async fn delete_product(db_pool: web::Data<DbPool>, request: web::Json<RequestDeleteProduct>) -> impl Responder {
    let conn = db_pool.get().unwrap();
    let request = request.into_inner();

    match get_auth_user(check_user_registration(&db_pool, request.login, &request.password).await) {
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
    }
}