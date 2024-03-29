use actix_web::{get, HttpResponse, Responder, web};
use actix_web::http::StatusCode;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{Client, header};
use reqwest::header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION, HeaderValue, HOST, REFERER, UPGRADE_INSECURE_REQUESTS, USER_AGENT};
use crate::models::JsonResponse;

// const CURRENT_USER_AGENT: &'static str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36";

// {'Accept': '*/*', 'Connection': 'keep-alive', 'User-Agent': 'Mozilla/5.0 (Windows NT 6.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/66.0.3359.158 Safari/537.36', 'Accept-Encoding': 'gzip, deflate, br', 'Accept-Language': 'en-US;q=0.5,en;q=0.3', 'Cache-Control': 'max-age=0', 'Upgrade-Insecure-Requests': '1', 'Referer': 'https://google.com'}


#[get("/get_logo/{name}")]
pub async fn find_logo(name: web::Path<String>) -> impl Responder {
    let name = name.into_inner();

    match get_logo_url(&name).await {
        Ok(v) => { HttpResponse::Ok().json(JsonResponse::new(v)) }
        Err(e) => { HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e) }
    }
}

pub async fn get_logo_url(name: &str) -> Result<Vec<String>, String> {

    let headers = header::HeaderMap::new();
    // headers.insert(HOST, "yandex.ru".parse().unwrap());
    // headers.insert(HOST, "google.com".parse().unwrap());
    // headers.insert(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36".parse().unwrap());
    // headers.insert(ACCEPT, "*/*".parse().unwrap());
    // headers.insert(CONNECTION, "keep-alive".parse().unwrap());
    // headers.insert(ACCEPT_ENCODING, "gzip, deflate, br".parse().unwrap());
    // headers.insert(ACCEPT_LANGUAGE, "en-US;q=0.5,en;q=0.3".parse().unwrap());
    // headers.insert(CACHE_CONTROL, "max-age=0".parse().unwrap());
    // headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());
    // headers.insert(REFERER, "https://google.com".parse().unwrap());
    // headers.insert(REFERER, "https://yandex.ru".parse().unwrap());

    // headers.insert("Host", "yandex.ru".parse().unwrap());
    // headers.insert("User-Agent", "curl/7.84.0".parse().unwrap());
    // headers.insert("Accept", "*/*".parse().unwrap());

    lazy_static! {
        static ref CLIENT: Client = reqwest::Client::builder().build().unwrap();
    }

    // let url = format!("http://yandex.ru/images/search?from=tabbar&text={}", name);
    let url = format!("https://www.google.ru/search?q={}&tbm=isch", name);

    // if let Ok(ok_client) = client {
    let res = match CLIENT.get(url).headers(headers).send().await {
        Ok(v) => { v }
        Err(err) => { return Err(err.to_string()); }
    }.text().await;

    let res = match res {
        Ok(v) => { v }
        Err(err) => { return Err(err.to_string()); }
    };

    // println!("request");

    lazy_static! {
        // static ref RE: Regex = Regex::new(r#""preview":\[\{"url":"([^"]+)"#).unwrap(); // yandex
        static ref RE: Regex = Regex::new(r#"class="yWs4tf" alt="" src="([^"]+)"#).unwrap(); // google
    }

    let caps = RE.captures_iter(res.as_str());

    let mut ans = Vec::new();

    for val in caps {
        let url = &val[1];
        if url.starts_with("http") {
            ans.push(url.to_string());
        } else {
            ans.push(format!("http://{url}"));
        }
    }

    // if res.contains("Нам очень жаль") {
    //     return HttpResponse::Ok().json(JsonResponse::new("the external server is overloaded"));
    // }


    return Ok(ans);

    // return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish();
}