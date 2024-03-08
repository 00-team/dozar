use std::env;

use actix_files as af;
use actix_web::{
    get, http::header::ContentType, web::Data, App, HttpResponse, HttpServer,
    Responder,
};
// use actix_web_httpauth::extractors::bearer;
use sqlx::{Pool, Sqlite, SqlitePool};
use utoipa::OpenApi;

mod api;
mod models;

pub struct AppState {
    db: Pool<Sqlite>,
}

// #[get("/")]
// async fn index(state: Data<AppState>) -> impl Responder {
//     let result = sqlx::query_as! {
//         models::Product,
//         "select * from products"
//     }
//     .fetch_all(&state.db)
//     .await;
//
//     match result {
//         Ok(products) => HttpResponse::Ok().json(products),
//         Err(e) => HttpResponse::InternalServerError()
//             .body(format!("fetch error: {e}")),
//     }
// }
//
// #[derive(Deserialize)]
// struct AddProduct {
//     title: String,
//     price: i64,
// }
//
// #[get("/add/")]
// async fn add(
//     state: Data<AppState>, query: Query<AddProduct>,
// ) -> impl Responder {
//     let created_at = chrono::Local::now().timestamp();
//     let result = sqlx::query_as! {
//         Product,
//         "insert into products (title, created_at, price) values(?, ?, ?)",
//         query.title,
//         created_at,
//         query.price
//     }
//     .execute(&state.db)
//     .await;
//
//     match result {
//         Ok(result) => HttpResponse::Ok()
//             .body(format!("id: {}", result.last_insert_rowid())),
//         Err(e) => HttpResponse::BadRequest().body(format!("sql error: {e}")),
//     }
// }

#[get("/openapi.json")]
async fn openapi() -> impl Responder {
    let mut openapi = api::user::ApiUserDoc::openapi();
    openapi.paths.paths = openapi
        .paths
        .paths
        .iter()
        .map(|(path, value)| {
            let path = api::user::ApiUserDoc::PATH.to_string() + path;
            let mut value = value.to_owned();
            value.operations.iter_mut().for_each(|(_, op)| {
                if let Some(tags) = &openapi.tags {
                    op.tags =
                        Some(tags.iter().map(|t| t.name.clone()).collect());
                }
            });

            (path, value)
        })
        .collect();

    HttpResponse::Ok().json(openapi)
}

#[get("/rapidoc")]
async fn rapidoc() -> impl Responder {
    HttpResponse::Ok().content_type(ContentType::html()).body(
        r###"<!doctype html>
    <html><head><meta charset="utf-8"><style>rapi-doc {
    --green: #00dc7d; --blue: #5199ff; --orange: #ff6b00;
    --red: #ec0f0f; --yellow: #ffd600; --purple: #782fef; }</style>
    <script type="module" src="/static/rapidoc.js"></script></head><body>
    <rapi-doc spec-url="/openapi.json" persist-auth="true"
    bg-color="#040404" text-color="#f2f2f2"
    header-color="#040404" primary-color="#ec0f0f"
    nav-text-color="#eee" font-size="largest"
    allow-spec-url-load="false" allow-spec-file-load="false"
    show-method-in-nav-bar="as-colored-block" response-area-height="500px"
    show-header="false" schema-expand-level="1" /></body> </html>"###,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::from_path("./secrets.env").expect("could not read secrets.env");
    pretty_env_logger::init();

    let pool = SqlitePool::connect(
        &env::var("DATABASE_URL").expect("DATABASE_URL was not found in env"),
    )
    .await
    .expect("sqlite pool initialization failed");

    sqlx::migrate!().run(&pool).await.expect("migration failed");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(openapi)
            .service(rapidoc)
            .service(af::Files::new("/static", "./static").show_files_listing())
            // .app_data(bearer::Config::default())
            .service(api::user::router())
    })
    .bind(("127.0.0.1", 7200))?
    .run()
    .await
}
