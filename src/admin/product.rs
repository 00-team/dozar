use actix_web::web::{Data, Json, Path, Query};
use actix_web::{delete, get, patch, post, HttpResponse, Responder, Scope};
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

use crate::docs::UpdatePaths;
use crate::models::{Admin, ListInput, Product};
use crate::utils::CutOff;
use crate::AppState;

#[derive(OpenApi)]
#[openapi(
    tags((name = "admin::product")),
    paths(product_list, product_get, product_add, product_update, product_delete),
    components(schemas(Product, ProductAddBody, ProductUpdateBody)),
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
    base_price: i64,
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
        Ok(v) => HttpResponse::Ok().json(Product {
            id: v.last_insert_rowid(),
            title: body.title.clone(),
            start: body.start,
            end: body.end,
            base_price: body.base_price,
            ..Default::default()
        }),
        Err(e) => {
            log::error!("err: {e}");
            HttpResponse::InternalServerError().body("db err")
        }
    }
}

#[derive(Deserialize, ToSchema)]
struct ProductUpdateBody {
    title: Option<String>,
    end: Option<i64>,
    start: Option<i64>,
    base_price: Option<i64>,

    detail: Option<String>,
    buy_now_opens: Option<i64>,
    buy_now_price: Option<i64>,
}

#[utoipa::path(
    patch,
    request_body = ProductUpdateBody,
    responses(
        (status = 200, body = Product)
    )
)]
/// Product Update
#[patch("/{id}/")]
async fn product_update(
    _: Admin, path: Path<(i64,)>, body: Json<ProductUpdateBody>,
    state: Data<AppState>,
) -> impl Responder {
    let mut change = false;
    let result = sqlx::query_as! {
        Product,
        "select * from products where id = ?",
        path.0
    }
    .fetch_optional(&state.sql)
    .await;

    let mut product = match result {
        Ok(v) => {
            if let Some(v) = v {
                v
            } else {
                return HttpResponse::NotFound().body("product not found");
            }
        }
        Err(e) => {
            log::error!("err: {e}");
            return HttpResponse::InternalServerError().body("db err");
        }
    };

    if let Some(title) = &body.title {
        change = true;
        product.title = title.to_string();
    }

    if let Some(bp) = body.base_price {
        change = true;
        product.base_price = bp;
    }

    if let Some(end) = body.end {
        change = true;
        product.end = end;
    }

    if let Some(start) = body.start {
        change = true;
        product.start = start;
    }

    if product.end != 0 && product.start >= product.end {
        return HttpResponse::BadRequest().body("invalid timing");
    }

    if let Some(detail) = &body.detail {
        change = true;
        product.detail = Some(detail.to_string());
    }

    if let Some(open) = body.buy_now_opens {
        change = true;
        product.buy_now_opens = if open < 0 { None } else { Some(open) };
    }

    if let Some(price) = body.buy_now_price {
        change = true;
        product.buy_now_price = if price < 0 { None } else { Some(price) };
    }

    if change {
        product.title.cut_off(100);
        product.detail.cut_off(2048);

        let _ = sqlx::query_as! {
            Product,
            "update products set
            title = ?, end = ?, start = ?, base_price = ?,
            detail = ?, buy_now_price = ?, buy_now_opens = ? where id = ?",
            product.title, product.end, product.start, product.base_price,
            product.detail, product.buy_now_price, product.buy_now_opens,
            product.id
        }
        .execute(&state.sql)
        .await;
    }

    HttpResponse::Ok().json(product)
}

#[utoipa::path(
    delete,
    params(("id" = i64, Path,)),
    responses(
        (status = 200, body = String)
    )
)]
/// Product Delete
#[delete("/{id}/")]
async fn product_delete(
    _: Admin, path: Path<(i64,)>, state: Data<AppState>,
) -> impl Responder {
    let result = sqlx::query_as! {
        Product,
        "delete from products where id = ?",
        path.0
    }
    .execute(&state.sql)
    .await;

    match result {
        Ok(v) => HttpResponse::Ok().body("ok"),
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
        .service(product_add)
        .service(product_update)
        .service(product_delete)
}
