use std::env;

use actix_web::{
    get,
    web::{Data, Query},
    App, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;
use sqlx::{types::chrono, Pool, Sqlite, SqlitePool};

mod models;
mod api;

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
            .service(api::user::router())
    })
    .bind(("127.0.0.1", 7200))?
    .run()
    .await
}
