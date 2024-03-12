use actix_web::web::{Data, Query};
use actix_web::{get, HttpResponse, Responder, Scope};
use utoipa::OpenApi;

use crate::docs::UpdatePaths;
use crate::models::{ListInput, Offer, Product, User};
use crate::AppState;

#[derive(OpenApi)]
#[openapi(
    tags((name = "api::product")),
    paths(product_list, product_get, offer_list),
    components(schemas(Offer)),
    servers((url = "/products")),
    modifiers(&UpdatePaths)
)]
pub struct Doc;

#[utoipa::path(
    get,
    params(("page" = u32, Query, example = 0)),
    responses(
        (status = 200, body = Vec<Product>)
    )
)]
/// Product List
#[get("/")]
async fn product_list(
    query: Query<ListInput>, state: Data<AppState>,
) -> impl Responder {
    let offset = i64::from(query.page) * 30;

    let result = sqlx::query_as! {
        Product,
        "select * from products limit 30 offset ?",
        offset
    }
    .fetch_all(&state.sql)
    .await;

    match result {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => {
            log::error!("err: {e}");
            HttpResponse::InternalServerError().body("db err")
        }
    }
}

#[utoipa::path(
    get,
    params(("id" = i64, Path,)),
    responses(
        (status = 200, body = Product)
    )
)]
/// Product Get
#[get("/{id}/")]
async fn product_get(_: User, product: Product) -> impl Responder {
    HttpResponse::Ok().json(product)
}

#[utoipa::path(
    get,
    params(
        ("id" = i64, Path,),
        ("page" = u32, Query,),
    ),
    responses(
        (status = 200, body = Vec<Offer>)
    )
)]
/// Product Offers List
#[get("/{id}/offers/")]
async fn offer_list(
    _: User, product: Product, query: Query<ListInput>, state: Data<AppState>,
) -> impl Responder {
    let offset = i64::from(query.page) * 30;

    let result = sqlx::query_as! {
        Offer,
        "select * from offers where product = ? limit 30 offset ?",
        product.id, offset
    }
    .fetch_all(&state.sql)
    .await;

    match result {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => {
            log::error!("err: {e}");
            HttpResponse::InternalServerError().body("db err")
        }
    }
}

pub fn router() -> Scope {
    Scope::new("/products")
        .service(product_list)
        .service(product_get)
        .service(offer_list)
}
