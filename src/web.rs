use actix_web::dev::HttpServiceFactory;
use actix_web::http::header::ContentType;
use actix_web::middleware::NormalizePath;
use actix_web::{get, routes, HttpResponse, Scope};
use std::fs::read_to_string;

#[routes]
#[get("/")]
#[get("/login")]
#[get("/products")]
async fn app_index() -> HttpResponse {
    let result = read_to_string("dist/index.html")
        .unwrap_or("err reading app index.html".to_string());
    HttpResponse::Ok().content_type(ContentType::html()).body(result)
}

#[get("/7531414.txt")]
async fn enamad_code() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::plaintext()).body("7531414")
}

pub fn router() -> impl HttpServiceFactory {
    Scope::new("")
        .wrap(NormalizePath::trim())
        .service(app_index)
        .service(enamad_code)
}
