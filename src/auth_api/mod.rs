use actix_web::{post, HttpResponse, Responder, web};
use actix_web::http::StatusCode;
use diesel::{EqAll, QueryDsl, RunQueryDsl};

use crate::data_base::DbPool;
use crate::models::{AuthUser, NewAuthUser, JsonResponse, User};
use crate::hashing_api::{hashing_password, verify_password};

use crate::auth_api::VerifyUserErr::{InternalError, InvalidLogin, InvalidPassword};
use crate::hashing_api::VerifyPasswordErr;
use crate::schema::auth;
use serde::Serialize;



#[post("/register")]
pub async fn register(db_pool: web::Data<DbPool>, user: web::Json<User>) -> impl Responder {

    let conn = db_pool.get().unwrap();
    let user = user.into_inner();

    let hash_password = match hashing_password(&user.password) {
        Ok(v) => { v }
        Err(_) => { return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish(); }
    };

    let new_user = NewAuthUser {
        login: user.login,
        hash_password,

    };

    match web::block(move || {
        diesel::insert_into(auth::table)
            .values(&new_user)
            .execute(&*conn)

    }).await {
        Ok(v) => {
            match v {
                Ok(_) => { HttpResponse::Ok().body("success") }
                Err(v) => {
                    if v.to_string().contains("duplicate key value violates unique constraint") {
                        return HttpResponse::Ok().json(JsonResponse::new("this entry already exists"));
                    }
                    HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish()
                }
            }}
        Err(_) => { HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish() }
    }

        // .get_result::<AuthUser>(&conn)
        // .expect("Error saving new user");
}



#[post("/check_registration")]
pub async fn check_registration(db_pool: web::Data<DbPool>, user: web::Json<User>) -> impl Responder {

    let user = user.into_inner();

    match get_auth_user(check_user_registration(&db_pool, user.login, &user.password).await) {
        Ok(_) => { HttpResponse::Ok().json(JsonResponse::new("registered")) }
        Err(e) => { e }
    }

}

pub enum VerifyUserErr {

    InvalidLogin,
    InvalidPassword,
    InternalError(String)

}

pub async fn check_user_registration(db_pool: &web::Data<DbPool>, login: String, password: &str) -> Result<AuthUser, VerifyUserErr> {

    let conn = db_pool.get().unwrap();

    let mut user = match web::block(move || {
        auth::dsl::auth
            .filter(auth::login.eq_all(login))
            .limit(1)
            .load::<AuthUser>(&*conn)
    }).await {
        Ok(v) => { match v {
            Ok(v) => { v }
            Err(e) => { return Err(InternalError(e.to_string())) }
        } }
        Err(_) => { return Err(InternalError(String::from("blocking error"))); }
    };

    if user.len() == 0 {
        return Err(InvalidLogin);
    } else {
        if let Err(err) = verify_password(password, &user[0].hash_password) {
            return match err {
                VerifyPasswordErr::InvalidPassword(_) => { Err(InvalidPassword) }
                VerifyPasswordErr::InternalError(v) => { Err(InternalError(v)) }
            }
        } else {

            Ok(user.swap_remove(0))
        }
    }
}

pub fn get_auth_user(user: Result<AuthUser, VerifyUserErr>) -> Result<AuthUser, HttpResponse>{
    match user {
        Ok(v) => { Ok(v) }
        Err(err) => {
            Err(match err {
                InvalidLogin => { HttpResponse::Ok().json(JsonResponse::new("user not found")) }
                InvalidPassword => { HttpResponse::Ok().json(JsonResponse::new("invalid password")) }
                InternalError(_) => { HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish() }
            })
        }
    }
}

