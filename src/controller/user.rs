use crate::service::{AuthService, UserService};
use crate::utils::Token;
use actix_web::{post, web, HttpResponse, Responder};
use mysql_async::Pool;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    token: String,
    tag: String,
    nonce: String,
}

#[post("/user/login")]
pub async fn login(
    pool: web::Data<Pool>,
    login_request: web::Json<LoginRequest>,
) -> impl Responder {
    let username = &login_request.username;
    let password = &login_request.password;
    match pool.get_conn().await {
        Ok(mut conn) => match UserService::login(&mut conn, username, password).await {
            Ok(token) => HttpResponse::Ok().json(LoginResponse {
                token: token.token,
                tag: token.tag,
                nonce: token.nonce,
            }),
            Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().json(e.to_string()),
    }
}

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
    name: String,
}

#[derive(Debug, Serialize)]
struct RegisterResponse {
    token: String,
    tag: String,
    nonce: String,
}

#[post("/user/register")]
pub async fn register(
    pool: web::Data<Pool>,
    register_request: web::Json<RegisterRequest>,
) -> impl Responder {
    let username = &register_request.username;
    let password = &register_request.password;
    let name = &register_request.name;
    match pool.get_conn().await {
        Ok(mut conn) => match UserService::register(&mut conn, username, password, name).await {
            Ok(token) => HttpResponse::Ok().json(RegisterResponse {
                token: token.token,
                tag: token.tag,
                nonce: token.nonce,
            }),
            Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct CreditRuleRequest {
    token: String,
    tag: String,
    nonce: String,
}

#[derive(Debug, Serialize)]
struct CreditRuleResponse {
    credit_level: u32,
    discount_percentage: String,
    overdraft_limit: String,
    auto_upgrade_balance: String,
    auto_upgrade_total_purchase: String,
}

#[post("/user/credit_rule")]
pub async fn credit_rule(
    pool: web::Data<Pool>,
    credit_rule_request: web::Json<CreditRuleRequest>,
) -> impl Responder {
    let request = credit_rule_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    match pool.get_conn().await {
        Ok(mut conn) => match UserService::get_credit_rule(&mut conn, token).await {
            Ok(credit_rule) => HttpResponse::Ok().json(CreditRuleResponse {
                credit_level: credit_rule.level,
                discount_percentage: credit_rule.discount_percentage.to_string(),
                overdraft_limit: credit_rule.overdraft_limit.to_string(),
                auto_upgrade_balance: credit_rule.upgrade_balance.to_string(),
                auto_upgrade_total_purchase: credit_rule.upgrade_purchase.to_string(),
            }),
            Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct UserDetailRequest {
    token: String,
    tag: String,
    nonce: String,
}

#[derive(Debug, Serialize)]
struct UserDetailResponse {
    username: String,
    name: String,
    address: String,
    email: String,
    account_balance: String,
    credit_level: u32,
    total_purchase: String,
    overdraft_limit: String,
}

#[post("/user/detail")]
pub async fn user_detail(
    pool: web::Data<Pool>,
    user_detail_request: web::Json<UserDetailRequest>,
) -> impl Responder {
    let request = user_detail_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    match pool.get_conn().await {
        Ok(mut conn) => match UserService::get_user_detail(&mut conn, token).await {
            Ok(user) => HttpResponse::Ok().json(UserDetailResponse {
                username: user.username,
                name: user.name,
                address: user.address,
                email: user.email,
                account_balance: user.account_balance.to_string(),
                credit_level: user.credit_level,
                total_purchase: user.total_purchase.to_string(),
                overdraft_limit: user.overdraft_limit.to_string(),
            }),
            Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().json(e.to_string()),
    }
}

#[derive(Debug, Serialize)]
struct UserProfileResponse {
    username: String,
    name: String,
    email: String,
    credit_level: u32,
}

#[post("/user/{username}/profile")]
pub async fn user_profile(pool: web::Data<Pool>, username: web::Path<(String,)>) -> impl Responder {
    let username = &username.into_inner().0;
    match pool.get_conn().await {
        Ok(mut conn) => match UserService::get_user_profile(&mut conn, username).await {
            Ok(user) => HttpResponse::Ok().json(UserProfileResponse {
                username: user.username,
                name: user.name,
                email: user.email,
                credit_level: user.credit_level,
            }),
            Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct UserLogoutRequest {
    token: String,
    tag: String,
    nonce: String,
}

#[derive(Debug, Serialize)]
struct UserLogoutResponse {
    message: String,
}

#[post("/user/logout")]
pub async fn user_logout(
    pool: web::Data<Pool>,
    user_logout_request: web::Json<UserLogoutRequest>,
) -> impl Responder {
    let request = user_logout_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    match pool.get_conn().await {
        Ok(mut conn) => match AuthService::logout_user(&mut conn, token).await {
            Ok(_) => HttpResponse::Ok().json(UserLogoutResponse {
                message: "logout successfully".to_string(),
            }),
            Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().json(e.to_string()),
    }
}
