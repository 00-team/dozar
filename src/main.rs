use std::{env, fs::read_to_string, os::unix::fs::PermissionsExt};

use actix_files as af;
use actix_web::{
    get,
    http::header::ContentType,
    web::{scope, Data},
    App, HttpResponse, HttpServer, Responder,
};
// use actix_web_httpauth::extractors::bearer;
use sqlx::{Pool, Sqlite, SqlitePool};
use utoipa::OpenApi;

mod api;
mod docs;
mod models;
mod utils;

use crate::docs::ApiDoc;

pub struct AppState {
    pub sql: Pool<Sqlite>,
}

#[get("/")]
async fn index() -> impl Responder {
    let result = read_to_string("dist/index.html")
        .unwrap_or("err reading index.html".to_string());
    HttpResponse::Ok().content_type(ContentType::html()).body(result)
}

#[get("/openapi.json")]
async fn openapi() -> impl Responder {
    let mut doc = ApiDoc::openapi();
    doc.merge(api::user::ApiUserDoc::openapi());
    doc.merge(api::verification::ApiVerificationDoc::openapi());

    HttpResponse::Ok().json(doc)
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

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { sql: pool.clone() }))
            .service(openapi)
            .service(rapidoc)
            .service(af::Files::new("/static", "./static").show_files_listing())
            .service(
                scope("/api")
                    .service(api::user::router())
                    .service(api::verification::verification),
            )
    });

    let server = if cfg!(debug_assertions) {
        server.bind(("127.0.0.1", 7200)).unwrap()
    } else {
        const PATH: &'static str = "/usr/share/nginx/sockets/dozar.sock";
        let s = server.bind_uds(PATH).unwrap();
        std::fs::set_permissions(PATH, std::fs::Permissions::from_mode(0o777))?;
        s
    };

    server.run().await
}
