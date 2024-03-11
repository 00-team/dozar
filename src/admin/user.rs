
use actix_web::web::{Data, Query};
// use actix_multipart::form::tempfile::TempFile;
// use actix_multipart::form::MultipartForm;
// use actix_web::web::{Data, Json, Query};
use actix_web::{get, HttpResponse, Responder, Scope};
// use serde::{Deserialize, Serialize};
// use sha2::{Digest, Sha512};
// use std::path::Path;
use utoipa::OpenApi;

// use crate::api::verification::verify;
// use crate::config::Config;
use crate::docs::UpdatePaths;
use crate::models::{Admin, ListInput, User};
use crate::AppState;
// use crate::utils::{get_random_bytes, get_random_string, save_photo};
// use crate::AppState;

#[derive(OpenApi)]
#[openapi(
    tags((name = "admin::user")),
    paths(user_list),
    components(schemas(User)),
    servers((url = "/users")),
    modifiers(&UpdatePaths)
)]
pub struct Doc;

#[utoipa::path(
    get,
    params(("page" = u32, Query,)),
    responses(
        (status = 200, body = Vec<User>)
    )
)]
#[get("/")]
async fn user_list(
    _: Admin, query: Query<ListInput>, state: Data<AppState>,
) -> impl Responder {
    let offset = i64::from(query.page) * 30;

    let result = sqlx::query_as! {
        User,
        "select * from users limit 30 offset ?",
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

pub fn router() -> Scope {
    Scope::new("/users").service(user_list)
}
