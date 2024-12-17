use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn homepage() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}
