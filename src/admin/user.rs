use actix_web::web::{Data, Json, Path, Query};
use actix_web::{get, patch, HttpResponse, Responder, Scope};
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

use crate::docs::UpdatePaths;
use crate::models::{Admin, ListInput, User};
use crate::utils::CutOff;
use crate::AppState;

#[derive(OpenApi)]
#[openapi(
    tags((name = "admin::user")),
    paths(user_list, user_get, user_update),
    components(schemas(User, AdminUpdateBody)),
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

#[utoipa::path(
    get,
    params(("id" = i64, Path,)),
    responses(
        (status = 200, body = User)
    )
)]
#[get("/{id}/")]
async fn user_get(
    _: Admin, path: Path<(i64,)>, state: Data<AppState>,
) -> impl Responder {
    let (id,) = path.into_inner();

    let result = sqlx::query_as! {
        User,
        "select * from users where id = ?",
        id
    }
    .fetch_optional(&state.sql)
    .await;

    match result {
        Ok(v) => {
            if let Some(v) = v {
                HttpResponse::Ok().json(v)
            } else {
                HttpResponse::NotFound().body("user not found")
            }
        }
        Err(e) => {
            log::error!("err: {e}");
            HttpResponse::InternalServerError().body("db err")
        }
    }
}

#[derive(Deserialize, ToSchema)]
struct AdminUpdateBody {
    banned: Option<bool>,
    name: Option<String>,
}

#[utoipa::path(
    patch,
    params(("id" = i64, Path,)),
    request_body = AdminUpdateBody,
    responses(
        (status = 200, body = User)
    )
)]
#[patch("/{id}/")]
async fn user_update(
    admin: Admin, path: Path<(i64,)>, body: Json<AdminUpdateBody>, state: Data<AppState>,
) -> impl Responder {
    let (id,) = path.into_inner();
    let mut change = false;

    let result = sqlx::query_as! {
        User,
        "select * from users where id = ?",
        id
    }
    .fetch_optional(&state.sql)
    .await;

    let mut user = match result {
        Ok(v) => {
            if let Some(v) = v {
                v
            } else {
                return HttpResponse::NotFound().body("user not found");
            }
        }
        Err(e) => {
            log::error!("err: {e}");
            return HttpResponse::InternalServerError().body("db err");
        }
    };

    if let Some(v) = &body.name {
        change = true;
        user.name = Some(v.clone());
    };

    if let Some(v) = &body.banned {
        if !user.admin && admin.id != user.id {
            change = true;
            user.banned = *v;
        }
    };

    if change {
        user.name.cut_off(100);

        let _ = sqlx::query_as! {
            User,
            "update users set name = ?, banned = ? where id = ?",
            user.name, user.banned, user.id
        }
        .execute(&state.sql)
        .await;
    }

    HttpResponse::Ok().json(user)
}

pub fn router() -> Scope {
    Scope::new("/users").service(user_list).service(user_get).service(user_update)
}
