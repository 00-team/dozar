use std::{env, os::unix::fs::PermissionsExt};

use crate::config::Config;
use crate::docs::{doc_add_prefix, ApiDoc};
use actix_files as af;
use actix_web::web::ServiceConfig;
use actix_web::{
    get,
    http::header::ContentType,
    middleware,
    web::{scope, Data},
    App, HttpResponse, HttpServer, Responder,
};
use sqlx::{Pool, Sqlite, SqlitePool};
use utoipa::OpenApi;

mod admin;
mod api;
mod config;
mod docs;
mod general;
mod models;
mod utils;
mod web;

pub struct AppState {
    pub sql: Pool<Sqlite>,
}

#[get("/openapi.json")]
async fn openapi() -> impl Responder {
    let mut doc = ApiDoc::openapi();
    doc.merge(api::user::ApiUserDoc::openapi());
    doc.merge(api::verification::ApiVerificationDoc::openapi());
    doc.merge(api::product::Doc::openapi());

    let mut admin_doc = ApiDoc::openapi();
    admin_doc.merge(admin::user::Doc::openapi());
    admin_doc.merge(admin::product::Doc::openapi());

    doc_add_prefix(&mut admin_doc, "/admin", false);

    doc.merge(admin_doc);

    doc_add_prefix(&mut doc, "/api", false);

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

fn config_static(app: &mut ServiceConfig) {
    if cfg!(debug_assertions) {
        app.service(af::Files::new("/static", "./static"));
        app.service(af::Files::new("/assets", "./dist/assets"));
        app.service(af::Files::new("/records", "./records"));
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::from_path("./secrets.env").expect("could not read secrets.env");
    pretty_env_logger::init();

    let _ = std::fs::create_dir(Config::RECORD_DIR);

    let pool = SqlitePool::connect(
        &env::var("DATABASE_URL").expect("DATABASE_URL was not found in env"),
    )
    .await
    .expect("sqlite pool initialization failed");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::new("%s %r %Ts"))
            .app_data(Data::new(AppState { sql: pool.clone() }))
            .configure(config_static)
            .service(openapi)
            .service(rapidoc)
            .service(
                scope("/api")
                    .service(api::user::router())
                    .service(api::product::router())
                    .service(api::verification::verification)
                    .service(
                        scope("/admin")
                            .service(admin::user::router())
                            .service(admin::product::router()),
                    ),
            )
            .service(web::router())
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
