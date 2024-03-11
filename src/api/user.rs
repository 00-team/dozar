use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use actix_web::web::{Data, Json, Query};
use actix_web::{
    delete, get, patch, post, put, HttpResponse, Responder, Scope,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::path::Path;
use utoipa::{OpenApi, ToSchema};

use crate::api::verification::verify;
use crate::config::Config;
use crate::docs::UpdatePaths;
use crate::models::{Action, Address, JsonStr, ListInput, Transaction, User};
use crate::utils::{get_random_bytes, get_random_string, save_photo};
use crate::AppState;

#[derive(OpenApi)]
#[openapi(
    tags((name = "api::user")),
    paths(
        login, user_get, user_update, user_update_photo,
        user_delete_photo, user_wallet_test, user_transactions_list
    ),
    components(schemas(
        User, LoginBody, UpdateBody, Address, UpdatePhoto, Transaction
    )),
    servers((url = "/user")),
    modifiers(&UpdatePaths)
)]
pub struct ApiUserDoc;

#[derive(Debug, Deserialize, ToSchema)]
struct LoginBody {
    phone: String,
    code: String,
}

#[utoipa::path(
    post,
    request_body = LoginBody,
    responses(
        (status = 200, body = User),
        (status = 400, body = String)
    )
)]
#[post("/login/")]
async fn login(body: Json<LoginBody>, state: Data<AppState>) -> impl Responder {
    if !verify(&body.phone, &body.code, Action::Login, &state.sql).await {
        return HttpResponse::BadRequest().body("invalid verification");
    }

    let token = get_random_string(Config::TOKEN_ABC, 69);
    let token_hashed = hex::encode(Sha512::digest(&token));

    let result = sqlx::query_as! {
        User,
        "select * from users where phone = ?",
        body.phone
    }
    .fetch_one(&state.sql)
    .await;

    let user: User = match result {
        Ok(mut v) => {
            v.token = token;

            let _ = sqlx::query_as! {
                User,
                "update users set token = ? where id = ?",
                token_hashed, v.id
            }
            .execute(&state.sql)
            .await;

            v
        }
        Err(_) => {
            let result = sqlx::query_as! {
                User,
                "insert into users (phone, token) values(?, ?)",
                body.phone, token_hashed
            }
            .execute(&state.sql)
            .await;

            User {
                phone: body.phone.clone(),
                token,
                id: result.unwrap().last_insert_rowid(),
                ..Default::default()
            }
        }
    };

    HttpResponse::Ok().json(user)
}

#[utoipa::path(
    get,
    responses(
        (status = 200, body = User)
    )
)]
#[get("/")]
async fn user_get(user: User) -> impl Responder {
    HttpResponse::Ok().json(user)
}

#[derive(Deserialize, ToSchema)]
struct UpdateBody {
    name: Option<String>,
    addr: Option<Address>,
}

#[utoipa::path(
    patch,
    request_body = UpdateBody,
    responses(
        (status = 200, body = User)
    )
)]
#[patch("/")]
async fn user_update(
    user: User, body: Json<UpdateBody>, state: Data<AppState>,
) -> impl Responder {
    let mut user = user;
    let mut change = false;
    if let Some(n) = &body.name {
        change = true;
        user.name = Some(n.clone());
    };

    if let Some(a) = &body.addr {
        change = true;
        user.addr = JsonStr(a.clone());
    };

    if change {
        if let Some(n) = user.name {
            user.name = Some(n[..255].to_string());
        }
        user.addr.country = user.addr.country[..255].to_string();
        user.addr.state = user.addr.state[..255].to_string();
        user.addr.city = user.addr.city[..255].to_string();
        user.addr.postal = user.addr.postal[..20].to_string();
        user.addr.detail = user.addr.detail[..2047].to_string();

        let _ = sqlx::query_as! {
            User,
            "update users set name = ? , addr = ? where id = ?",
            user.name, user.addr, user.id
        }
        .execute(&state.sql)
        .await;
    }

    HttpResponse::Ok().json(user)
}

#[derive(Debug, MultipartForm, ToSchema)]
struct UpdatePhoto {
    #[schema(value_type = String, format = Binary)]
    #[multipart(limit = "8 MiB")]
    photo: TempFile,
}

#[utoipa::path(
    put,
    request_body(content = UpdatePhoto, content_type = "multipart/form-data"),
    responses(
        (status = 200, body = User)
    )
)]
#[put("/photo/")]
async fn user_update_photo(
    user: User, form: MultipartForm<UpdatePhoto>, state: Data<AppState>,
) -> impl Responder {
    let mut user = user;

    let salt = if let Some(p) = &user.photo {
        p.clone()
    } else {
        let s = get_random_bytes(8);
        user.photo = Some(s.clone());
        s
    };

    let filename = format!("{}-{salt}", user.id);

    match save_photo(form.photo.file.path(), &filename) {
        Ok(_) => {}
        Err(e) => {
            log::error!("{e}");
            return HttpResponse::BadRequest().body("invalid photo");
        }
    }

    let _ = sqlx::query_as! {
        User,
        "update users set photo = ? where id = ?",
        user.photo, user.id
    }
    .execute(&state.sql)
    .await;

    HttpResponse::Ok().json(user)
}

#[utoipa::path(
    delete,
    responses(
        (status = 200)
    )
)]
#[delete("/photo/")]
async fn user_delete_photo(
    user: User, state: Data<AppState>,
) -> impl Responder {
    let mut user = user;

    if user.photo.is_none() {
        return HttpResponse::Ok();
    }

    let _ = std::fs::remove_file(Path::new(Config::RECORD_DIR).join(format!(
        "{}-{}",
        user.id,
        user.photo.unwrap()
    )));
    user.photo = None;

    let _ = sqlx::query_as! {
        User,
        "update users set photo = ? where id = ?",
        user.photo, user.id
    }
    .execute(&state.sql)
    .await;

    HttpResponse::Ok()
}

#[utoipa::path(
    post,
    responses(
        (status = 200)
    )
)]
#[post("/wallet/")]
async fn user_wallet_test(user: User, _state: Data<AppState>) -> impl Responder {
    let client = awc::Client::new();
    let request = client
        .post("https://api.idpay.ir/v1.1/payment")
        .insert_header(("X-SANDBOX", "1"))
        .insert_header(("X-API-KEY", "..."));

    #[derive(Serialize, Debug)]
    struct Data {
        order_id: String,
        amount: u64,
        name: Option<String>,
        phone: String,
        desc: String,
        callback: String,
    }

    let result = request
        .send_json(&Data {
            order_id: "12".to_string(),
            amount: 12002,
            name: None,
            phone: user.phone,
            desc: "Adding to Wallet".to_string(),
            callback: "http://127.0.0.1:7200/user/wallet/cb/".to_string(),
        })
        .await;

    match result {
        Ok(mut v) => {
            println!("status: {}", v.status());
            let body = v.json::<serde_json::Value>().await;
            println!("{:?} {:?}", v, body);
        }
        Err(e) => {
            log::error!("{}", e);
            return HttpResponse::InternalServerError();
        }
    }

    HttpResponse::Ok()
}


#[utoipa::path(
    get,
    params(("page" = u32, Query,)),
    responses(
        (status = 200, body = Vec<Transaction>)
    )
)]
#[get("/transactions/")]
async fn user_transactions_list(
    user: User, query: Query<ListInput>, state: Data<AppState>,
) -> impl Responder {
    let offset = query.page * 30;
    let result = sqlx::query_as! {
        Transaction,
        "select * from transactions where user = ? limit 30 offset ?",
        user.id, offset
    }
    .fetch_all(&state.sql)
    .await;

    match result {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => {
            log::error!("error: {e}");
            HttpResponse::InternalServerError().body("database failed")
        }
    }
}

pub fn router() -> Scope {
    Scope::new("/user")
        .service(login)
        .service(user_get)
        .service(user_update)
        .service(user_update_photo)
        .service(user_delete_photo)
        .service(user_wallet_test)
        .service(user_transactions_list)
}
