use actix_web::{get, post, HttpResponse, Responder, Scope};
use utoipa::OpenApi;

use crate::models::User;

#[derive(OpenApi)]
#[openapi(
    tags((name = "api::user")),
    paths(login, user_get), 
    components(schemas(User))
)]
pub struct ApiUserDoc;

impl ApiUserDoc {
    pub const PATH: &'static str = "/user";
}

#[utoipa::path(
    post,
    responses(
        (status = 200, body = User)
    )
)]
#[post("/login/")]
async fn login() -> impl Responder {
    HttpResponse::Ok()
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
    Scope::new(ApiUserDoc::PATH).service(login).service(user_get)
}
