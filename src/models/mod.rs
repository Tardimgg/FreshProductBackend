use diesel::{Insertable, Queryable};
use crate::schema::auth;
use crate::schema::products;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct User {
    pub login: String,
    pub password: String
}


#[derive(Queryable)]
pub struct AuthUser {
    pub user_id: i32,
    pub login: String,
    pub hash_password: String
}

#[derive(Insertable)]
#[table_name="auth"]
pub struct NewAuthUser {
    pub login: String,
    pub hash_password: String
}

#[derive(Queryable)]
pub struct Product {
    pub value_id: i32,
    pub user_id: i32,
    pub product_id_on_device: i32,
    pub image_url: String,
    pub product_title: String,
    pub product_subtitle: String,
    pub expiration_date: i64,
    pub start_tracking_date: i64
}

impl Product {
    pub fn cutting(self) -> CroppedProduct {
        CroppedProduct {
            product_id_on_device: self.product_id_on_device,
            image_url: self.image_url,
            product_title: self.product_title,
            product_subtitle: self.product_subtitle,
            expiration_date: self.expiration_date,
            start_tracking_date: self.start_tracking_date
        }
    }
}

#[derive(Insertable)]
#[table_name="products"]
pub struct NewProduct {
    pub user_id: i32,
    pub product_id_on_device: i32,
    pub image_url: String,
    pub product_title: String,
    pub product_subtitle: String,
    pub expiration_date: i64,
    pub start_tracking_date: i64
}

#[derive(Serialize)]
pub struct CroppedProduct {
    pub product_id_on_device: i32,
    pub image_url: String,
    pub product_title: String,
    pub product_subtitle: String,
    pub expiration_date: i64,
    pub start_tracking_date: i64
}

#[derive(Serialize)]
pub struct JsonResponse<T: Serialize> {
    answer: T
}

impl<T: Serialize> JsonResponse<T> {

    pub fn new(v: T) -> JsonResponse<T>{
        JsonResponse {
            answer: v
        }
    }
}