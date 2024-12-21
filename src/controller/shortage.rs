use crate::service::ShortageService;
use crate::utils::Token;
use actix_web::{post, web, HttpResponse, Responder};
use mysql_async::Pool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct ShortageCreateRequest {
    book_suppliers: Vec<(u32, u32, u32)>,
    token: String,
    tag: String,
    nonce: String,
}

#[derive(Debug, Serialize)]
struct ShortageCreateResponse {
    shortage_id: u32,
}

#[post("/shortage/create")]
pub async fn shortage_create(
    pool: web::Data<Pool>,
    book_shortage_create_request: web::Json<ShortageCreateRequest>,
) -> impl Responder {
    let request = book_shortage_create_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    let book_suppliers = &request.book_suppliers;
    let mut conn = pool.get_conn().await.unwrap();

    match ShortageService::create_book_shortage(&mut conn, token, book_suppliers).await {
        Ok(shortage_id) => HttpResponse::Ok().json(ShortageCreateResponse { shortage_id }),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
