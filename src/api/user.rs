use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpResponse, Responder, Scope};
use rand::Rng;
use serde::Deserialize;
use sha2::{Digest, Sha512};
use utoipa::{OpenApi, ToSchema};

use crate::api::verification::verify;
use crate::docs::UpdatePaths;
use crate::models::{Action, User};
use crate::AppState;

#[derive(OpenApi)]
#[openapi(
    tags((name = "api::user")),
    paths(login, user_get),
    components(schemas(User, LoginBody)),
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

    let token = get_random_token();
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

pub fn router() -> Scope {
    Scope::new("/user").service(login).service(user_get)
}

fn get_random_token() -> String {
    let mut rng = rand::thread_rng();

    const CHARSET: &[u8] = b"!@#$%^&*_+abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*_+";

    (0..69).map(|_| CHARSET[rng.gen_range(0..CHARSET.len())] as char).collect()
}
