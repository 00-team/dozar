use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use utoipa::{OpenApi, ToSchema};

use crate::{config::Config, models, utils::get_random_string};
use crate::{
    models::Action,
    utils::{self, phone_validator},
    AppState,
};

#[derive(OpenApi)]
#[openapi(
    paths(verification),
    components(schemas(VerificationData, VerificationResponse, Action))
)]
pub struct ApiVerificationDoc;

#[derive(ToSchema, Deserialize, Debug)]
struct VerificationData {
    phone: String,
    action: Action,
}

#[derive(ToSchema, Serialize, Debug)]
struct VerificationResponse {
    expires: i64,
    action: Action,
}

#[utoipa::path(
    post,
    request_body = VerificationData,
    responses(
        (status = 200, body = VerificationResponse)
    )
)]
#[post("/verification/")]
async fn verification(
    body: Json<VerificationData>, state: Data<AppState>,
) -> impl Responder {
    if !phone_validator(&body.phone) {
        return HttpResponse::BadRequest().body("invalid phone number");
    }

    let result = sqlx::query_as! {
        models::Verification,
        "select * from verifications where phone = ?",
        body.phone
    }
    .fetch_one(&state.sql)
    .await;

    let now = utils::now();
    match result {
        Ok(v) => {
            let t = v.expires - now;
            if t > 0 {
                return HttpResponse::Ok().json(VerificationResponse {
                    expires: t,
                    action: v.action,
                });
            }
        }
        Err(_) => {}
    }

    let _ = sqlx::query! {
        "delete from verifications where phone = ? or expires < ?",
        body.phone,
        now
    }
    .execute(&state.sql)
    .await;

    let code = get_random_string(Config::CODE_ABC, 5);
    log::info!("code: {code}");

    let action = match &body.action {
        Action::Login => "login",
        Action::Delete => "delete",
    };
    let expires = now + 180;
    let _ = sqlx::query_as! {
        models::Verification,
        "insert into verifications (phone, action, code, expires) values(?, ?, ?, ?)",
        body.phone, action, code, expires
    }.execute(&state.sql).await;

    HttpResponse::Ok().json(VerificationResponse {
        expires: 180,
        action: body.action.to_owned(),
    })
}

pub async fn verify(
    phone: &str, code: &str, action: Action, sql: &Pool<Sqlite>,
) -> bool {
    let result = sqlx::query_as! {
        models::Verification,
        "select * from verifications where phone = ?",
        phone
    }
    .fetch_one(sql)
    .await;

    let now = utils::now();

    match result {
        Ok(v) => {
            let tries = v.tries + 1;
            if v.expires <= now {
                let _ = sqlx::query! {
                    "delete from verifications where phone = ? or expires < ?",
                    phone, now
                }
                .execute(sql)
                .await;
                return false;
            }

            if v.action != action {
                return false;
            }

            if v.code != code {
                if tries > 2 {
                    return false;
                }

                let _ = sqlx::query! {
                    "update verifications set tries = ? where id = ?",
                    tries, v.id
                }
                .execute(sql)
                .await;

                return false;
            }

            let _ = sqlx::query! {
                "delete from verifications where phone = ? or expires < ?",
                phone, now
            }
            .execute(sql)
            .await;
            true
        }
        Err(_) => false,
    }
}
