use actix_web::{post, HttpResponse, Responder, Scope};


#[post("/login/")]
async fn login() -> impl Responder {
    HttpResponse::Ok()
}


pub fn router() -> Scope {
    Scope::new("/user").service(login)
}

