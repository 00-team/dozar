use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpResponse, Responder, Scope};
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

use crate::models::{Action, User};
use crate::docs::UpdatePaths;
use crate::AppState;
use crate::api::verification::verify;

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
    code: String
}

#[utoipa::path(
    post,
    request_body = LoginBody,
    responses(
        (status = 200, body = User)
    )
)]
#[post("/login/")]
async fn login(body: Json<LoginBody>, state: Data<AppState>) -> impl Responder {
    if !verify(&body.phone, &body.code, Action::Login, &state.sql).await {
        return HttpResponse::BadRequest().body("invalid verification");
    }

    HttpResponse::Ok().body("hi")
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
