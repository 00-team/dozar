use actix_web::web::{Data, Json, Path, Query};
use actix_web::{get, post, HttpResponse, Responder, Scope};
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

use crate::docs::UpdatePaths;
use crate::models::{Admin, ListInput, Product};
use crate::utils::CutOff;
use crate::AppState;

#[derive(OpenApi)]
#[openapi(
    tags((name = "admin::product")),
    paths(product_list, product_get, product_add),
    components(schemas(Product, ProductAddBody)),
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
    _: Admin, query: Query<ListInput>, state: Data<AppState>,
) -> impl Responder {
    let offset = i64::from(query.page) * 30;

    log::info!("offset: {offset}");
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
async fn product_get(
    _: Admin, path: Path<(i64,)>, state: Data<AppState>,
) -> impl Responder {
    let result = sqlx::query_as! {
        Product,
        "select * from products where id = ?",
        path.0
    }
    .fetch_optional(&state.sql)
    .await;

    match result {
        Ok(v) => {
            if let Some(v) = v {
                HttpResponse::Ok().json(v)
            } else {
                HttpResponse::NotFound().body("product not found")
            }
        }
        Err(e) => {
            log::error!("err: {e}");
            HttpResponse::InternalServerError().body("db err")
        }
    }
}


#[derive(Deserialize, ToSchema)]
struct ProductAddBody {
    title: String,
    end: i64,
    start: i64,
    base_price: i64
}

#[utoipa::path(
    post,
    request_body = ProductAddBody,
    responses(
        (status = 200, body = Product)
    )
)]
/// Product Add
#[post("/")]
async fn product_add(
    _: Admin, body: Json<ProductAddBody>, state: Data<AppState>,
) -> impl Responder {
    let mut body = body;
    body.title.cut_off(100);
    if body.end != 0 && body.start >= body.end {
        return HttpResponse::BadRequest().body("invalid start and end times");
    }

    let result = sqlx::query_as! {
        Product,
        "insert into products(title, end, start, base_price) values(?, ?, ?, ?)",
        body.title, body.end, body.start, body.base_price
    }
    .execute(&state.sql)
    .await;

    match result {
        Ok(v) => {
            HttpResponse::Ok().json(Product {
                id: v.last_insert_rowid(),
                title: body.title.clone(),
                start: body.start,
                end: body.end,
                base_price: body.base_price,
                ..Default::default()
            })
        }
        Err(e) => {
            log::error!("err: {e}");
            HttpResponse::InternalServerError().body("db err")
        }
    }
}

pub fn router() -> Scope {
    Scope::new("/products").service(product_list).service(product_get).service(product_add)
}
