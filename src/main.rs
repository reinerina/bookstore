use actix_web::{get, web, App, HttpResponse, HttpServer};
use bookstore::controller::{
    admin_login, admin_register, book_authors_search, book_detail, book_keywords_search, book_list,
    book_title_search, credit_rule, login, order_create, order_detail, order_history,
    purchase_order_detail, purchase_order_list, register, supplier_profile, user_detail,
    user_logout, user_update,
};
use mysql_async::prelude::{Query, WithParams};
use mysql_async::{OptsBuilder, Pool};
use serde::Deserialize;
use std::env;
use std::ops::Index;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[derive(Deserialize)]
struct Echo {
    content: String,
}

#[get("/echo")]
async fn echo_query(q: web::Query<Echo>) -> HttpResponse {
    HttpResponse::Ok().body(format!("q: {:?}", q.content))
}

#[get("/user/{id}/profile")]
async fn user_profile(info: web::Path<(u32,)>) -> HttpResponse {
    let user_id = info.into_inner().0;
    let response = format!("User profile of ID: {}", user_id);
    HttpResponse::Ok().body(response)
}

#[get("/user")]
async fn sql_user(conn: web::Data<Pool>) -> HttpResponse {
    let mut conn = conn.get_conn().await.unwrap();
    let query = r"SELECT USER()";
    let user = query.with(()).map(&mut conn, |s: String| s).await.unwrap();
    HttpResponse::Ok().body(user.index(0).clone())
}

async fn manual_hello() -> HttpResponse {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let opts = OptsBuilder::default()
        .db_name(env::var("SQL_DB").ok())
        .user(env::var("SQL_USER").ok())
        .pass(env::var("SQL_PWD").ok())
        .ip_or_hostname(env::var("SQL_HOSTNAME").unwrap_or("localhost".to_string()))
        .tcp_port(
            env::var("SQL_PORT")
                .unwrap_or("3306".to_string())
                .parse()
                .unwrap_or(3306),
        );

    let pool = Pool::new(opts);

    let pool_clone = pool.clone();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool_clone.clone()))
            .service(index)
            .service(echo_query)
            .route("/hey", web::get().to(manual_hello))
            .service(user_profile)
            .service(actix_files::Files::new("/assets/images", "assets/images"))
            .service(admin_register)
            .service(admin_login)
            .service(register)
            .service(login)
            .service(user_detail)
            .service(user_profile)
            .service(user_update)
            .service(user_logout)
            .service(supplier_profile)
            .service(credit_rule)
            .service(book_detail)
            .service(book_list)
            .service(book_title_search)
            .service(book_keywords_search)
            .service(book_authors_search)
            .service(order_detail)
            .service(order_history)
            .service(order_create)
            .service(purchase_order_list)
            .service(purchase_order_detail)
    });
    server.bind(("127.0.0.1", 8080))?.run().await?;

    match pool.disconnect().await {
        Ok(_) => log::info!("database connection pool disconnected"),
        Err(e) => log::error!("error disconnecting database connection pool: {}", e),
    }

    Ok(())
}
