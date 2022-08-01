use actix_web::{get, post, HttpResponse, Responder, web};
use lazy_static::lazy_static;
use pyo3::Python;
use pyo3::types::PyModule;

use serde::{Deserialize};
use crate::models::JsonResponse;

#[derive(Deserialize)]
pub struct Receipt {
    fn_param: String,
    fd: String,
    fp: String,
    total_sum: String,
    date: String,
    time: String,
    receipt_type: String
}

#[get("/echo/{q}")]
async fn echo(req_body: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(req_body.into_inner())
}

lazy_static! {
    static ref CODE: &'static str = include_str!("find_info.py");
}

#[post("/get_receipt_info")]
pub async fn get_receipt_info(info: web::Json<Receipt>) -> impl Responder {

    let gil = Python::acquire_gil();
    let py = gil.python();



    let activators = PyModule::from_code(py, CODE.clone(),
                                                    "find_info.py",
                                                    "find_info" );

    if let Ok(activators) = activators {

        let param = (
            &info.fn_param,
            &info.fd,
            &info.fp,
            &info.total_sum,
            &info.date,
            &info.time,
            &info.receipt_type
        );
        let res = activators.call_method1("get_receipt_info", param).unwrap().extract::<Vec<String>>().unwrap();


        match res.len() {
            0 => HttpResponse::NotFound().body("No found"),
            _ => HttpResponse::Ok().body(JsonResponse::new(res))
        }


    } else {
        HttpResponse::Ok().body("Python error")
    }
}