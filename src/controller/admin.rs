use crate::service::AdminService;
use crate::utils::Token;
use actix_web::{post, web, HttpResponse, Responder};
use mysql_async::Pool;
use serde::{Deserialize, Serialize};
use std::mem::take;

#[derive(Debug, Deserialize)]
struct AdminRegisterRequest {
    username: String,
    password: String,
    role: String,
    token: String,
    tag: String,
    nonce: String,
}

#[derive(Debug, Serialize)]
struct AdminRegisterResponse {
    admin_id: u32,
}

#[post("/admin/register")]
pub async fn admin_register(
    pool: web::Data<Pool>,
    mut admin_register_request: web::Json<AdminRegisterRequest>,
) -> impl Responder {
    let token = &Token {
        token: take(&mut admin_register_request.token),
        tag: take(&mut admin_register_request.tag),
        nonce: take(&mut admin_register_request.nonce),
    };
    let username = &admin_register_request.username;
    let password = &admin_register_request.password;
    let role = &admin_register_request.role;

    let mut conn = pool.get_conn().await.unwrap();

    match AdminService::register(&mut conn, username, password, role.parse().unwrap(), token).await
    {
        Ok(admin_id) => HttpResponse::Ok().json(AdminRegisterResponse { admin_id }),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct AdminLoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct AdminLoginResponse {
    token: String,
    tag: String,
    nonce: String,
}

#[post("/admin/login")]
pub async fn admin_login(
    pool: web::Data<Pool>,
    admin_login_request: web::Json<AdminLoginRequest>,
) -> impl Responder {
    let username = &admin_login_request.username;
    let password = &admin_login_request.password;
    match pool.get_conn().await {
        Ok(mut conn) => match AdminService::login(&mut conn, username, password).await {
            Ok(token) => HttpResponse::Ok().json(AdminLoginResponse {
                token: token.token,
                tag: token.tag,
                nonce: token.nonce,
            }),
            Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().json(e.to_string()),
    }
}
