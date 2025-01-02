use actix_web::{get, web, App, HttpResponse, HttpServer};
use bookstore::controller::{
    admin_book_add, admin_book_detail, admin_book_update, admin_customer_balance,
    admin_customer_credit, admin_customer_list, admin_detail, admin_location_list, admin_login,
    admin_order_list, admin_order_ship_auto, admin_register, admin_shortage_detail,
    admin_shortage_list, admin_stock_change, admin_stock_transfer, admin_user_search, author_list,
    book_authors_search, book_detail, book_keywords_search, book_list, book_title_search,
    credit_rule, keyword_list, login, order_create, order_detail, order_history, order_payment,
    publisher_list, purchase_order_create, purchase_order_detail, purchase_order_list, register,
    series_list, shortage_create, supplier_list, supplier_profile, user_detail, user_logout,
    user_profile, user_update,
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

#[get("/user")]
async fn sql_user(conn: web::Data<Pool>) -> HttpResponse {
    let mut conn = conn.get_conn().await.unwrap();
    let query = r"SELECT USER()";
    let user = query.with(()).map(&mut conn, |s: String| s).await.unwrap();
    HttpResponse::Ok().body(user.index(0).clone())
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
            .service(actix_files::Files::new("/assets/images", "assets/images"))
            .service(admin_register)
            .service(admin_login)
            .service(admin_detail)
            .service(admin_book_detail)
            .service(admin_location_list)
            .service(admin_stock_change)
            .service(admin_stock_transfer)
            .service(admin_book_update)
            .service(admin_book_add)
            .service(admin_customer_list)
            .service(admin_customer_credit)
            .service(admin_customer_balance)
            .service(admin_order_list)
            .service(admin_order_ship_auto)
            .service(admin_shortage_list)
            .service(admin_shortage_detail)
            .service(admin_user_search)
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
            .service(author_list)
            .service(publisher_list)
            .service(keyword_list)
            .service(supplier_list)
            .service(series_list)
            .service(book_title_search)
            .service(book_keywords_search)
            .service(book_authors_search)
            .service(order_detail)
            .service(order_history)
            .service(order_create)
            .service(order_payment)
            .service(purchase_order_list)
            .service(purchase_order_detail)
            .service(purchase_order_create)
            .service(shortage_create)
    });
    server.bind(("127.0.0.1", 8080))?.run().await?;

    match pool.disconnect().await {
        Ok(_) => log::info!("database connection pool disconnected"),
        Err(e) => log::error!("error disconnecting database connection pool: {}", e),
    }

    Ok(())
}
