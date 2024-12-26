use crate::service::{AdminService, StockService};
use crate::utils::Token;
use actix_web::{post, web, HttpResponse, Responder};
use mysql_async::Pool;
use serde::{Deserialize, Serialize};

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
    admin_register_request: web::Json<AdminRegisterRequest>,
) -> impl Responder {
    let request = admin_register_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    let username = &request.username;
    let password = &request.password;
    let role = &request.role;

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
    let request = admin_login_request.into_inner();
    let username = &request.username;
    let password = &request.password;
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

#[derive(Debug, Deserialize)]
struct AdminDetailRequest {
    token: String,
    tag: String,
    nonce: String,
}

#[derive(Debug, Serialize)]
struct AdminDetailResponse {
    admin_id: u32,
    username: String,
    role: String,
}

#[post("/admin/detail")]
pub async fn admin_detail(
    pool: web::Data<Pool>,
    admin_detail_request: web::Json<AdminDetailRequest>,
) -> impl Responder {
    let request = admin_detail_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };

    let mut conn = pool.get_conn().await.unwrap();

    match AdminService::get_admin_detail(&mut conn, token).await {
        Ok(admin) => HttpResponse::Ok().json(AdminDetailResponse {
            admin_id: admin.id,
            username: admin.username,
            role: admin.role.to_string(),
        }),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

#[derive(Debug, Serialize)]
struct AuthorDetailResponse {
    author_id: u32,
    name: String,
}

#[derive(Debug, Serialize)]
struct KeywordDetailResponse {
    keyword_id: u32,
    keyword: String,
}

#[derive(Debug, Serialize)]
struct PublisherDetailResponse {
    publisher_id: u32,
    name: String,
}

#[derive(Debug, Serialize)]
struct SupplierDetailResponse {
    supplier_id: u32,
    name: String,
    telephone: String,
    email: String,
    address: String,
    fax: String,
}

#[derive(Debug, Serialize)]
struct SeriesDetailResponse {
    series_id: u32,
    name: String,
    column: u32,
}

#[derive(Debug, Serialize)]
struct LocationDetailResponse {
    pub id: u32,
    pub book_id: u32,
    pub description: String,
    pub quantity: u32,
}

#[derive(Debug, Serialize)]
struct BookDetailResponse {
    book_id: u32,
    isbn: String,
    title: String,
    authors: Vec<AuthorDetailResponse>,
    publisher: PublisherDetailResponse,
    suppliers: Vec<SupplierDetailResponse>,
    in_series: Vec<SeriesDetailResponse>,
    price: String,
    keywords: Vec<KeywordDetailResponse>,
    locations: Vec<LocationDetailResponse>,
    catalog: String,
    cover: String,
    is_onstore: bool,
}

#[derive(Debug, Deserialize)]
struct BookDetailRequest {
    token: String,
    tag: String,
    nonce: String,
    book_id: u32,
}

#[post("/admin/book/detail")]
pub async fn admin_book_detail(
    pool: web::Data<Pool>,
    book_detail_request: web::Json<BookDetailRequest>,
) -> impl Responder {
    let request = book_detail_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    let book_id = request.book_id;

    let mut conn = pool.get_conn().await.unwrap();

    match AdminService::get_book_detail(&mut conn, token, book_id).await {
        Ok((book, locations)) => HttpResponse::Ok().json(BookDetailResponse {
            book_id: book.id,
            isbn: book.isbn,
            title: book.title,
            authors: book
                .authors
                .into_iter()
                .map(|author| AuthorDetailResponse {
                    author_id: author.id,
                    name: author.name,
                })
                .collect(),
            publisher: PublisherDetailResponse {
                publisher_id: book.publisher.id,
                name: book.publisher.name,
            },
            suppliers: book
                .suppliers
                .into_iter()
                .map(|supplier| SupplierDetailResponse {
                    supplier_id: supplier.id,
                    name: supplier.name,
                    telephone: supplier.telephone,
                    email: supplier.email,
                    address: supplier.address,
                    fax: supplier.fax,
                })
                .collect(),
            in_series: book
                .in_series
                .into_iter()
                .map(|series| SeriesDetailResponse {
                    series_id: series.series_id,
                    name: series.title,
                    column: series.column,
                })
                .collect(),
            price: book.price.to_string(),
            keywords: book
                .keywords
                .into_iter()
                .map(|keyword| KeywordDetailResponse {
                    keyword_id: keyword.id,
                    keyword: keyword.keyword,
                })
                .collect(),
            locations: locations
                .into_iter()
                .map(|location| LocationDetailResponse {
                    id: location.id,
                    book_id: location.book_id,
                    description: location.description,
                    quantity: location.quantity,
                })
                .collect(),
            catalog: book.catalog,
            cover: book.cover,
            is_onstore: book.is_onstore,
        }),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct LocationListRequest {
    token: String,
    tag: String,
    nonce: String,
}

#[derive(Debug, Serialize)]
struct LocationListItemResponse {
    id: u32,
    description: String,
}

#[derive(Debug, Serialize)]
struct LocationListResponse {
    locations: Vec<LocationListItemResponse>,
}

#[post("/admin/location/list")]
pub async fn admin_location_list(
    pool: web::Data<Pool>,
    location_list_request: web::Json<LocationListRequest>,
) -> impl Responder {
    let request = location_list_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };

    let mut conn = pool.get_conn().await.unwrap();

    match StockService::get_location_list(&mut conn, token).await {
        Ok(locations) => HttpResponse::Ok().json(LocationListResponse {
            locations: locations
                .into_iter()
                .map(|location| LocationListItemResponse {
                    id: location.id,
                    description: location.description,
                })
                .collect(),
        }),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct StockChangeRequest {
    token: String,
    tag: String,
    nonce: String,
    book_id: u32,
    location_id: u32,
    quantity: i32,
}

#[derive(Debug, Serialize)]
struct StockChangeResponse {
    message: String,
}

#[post("/admin/stock/change")]
pub async fn admin_stock_change(
    pool: web::Data<Pool>,
    stock_change_request: web::Json<StockChangeRequest>,
) -> impl Responder {
    let request = stock_change_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    let book_id = request.book_id;
    let location_id = request.location_id;
    let quantity = request.quantity;

    let mut conn = pool.get_conn().await.unwrap();

    match StockService::change_stock(&mut conn, token, book_id, location_id, quantity).await {
        Ok(_) => HttpResponse::Ok().json(StockChangeResponse {
            message: "stock change successfully".to_string(),
        }),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct StockTransferRequest {
    token: String,
    tag: String,
    nonce: String,
    book_id: u32,
    from_location_id: u32,
    to_location_id: u32,
    quantity: u32,
}

#[derive(Debug, Serialize)]
struct StockTransferResponse {
    message: String,
}

#[post("/admin/stock/transfer")]
pub async fn admin_stock_transfer(
    pool: web::Data<Pool>,
    stock_transfer_request: web::Json<StockTransferRequest>,
) -> impl Responder {
    let request = stock_transfer_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    let book_id = request.book_id;
    let from_location_id = request.from_location_id;
    let to_location_id = request.to_location_id;
    let quantity = request.quantity;

    let mut conn = pool.get_conn().await.unwrap();

    match StockService::transfer_stock(
        &mut conn,
        token,
        book_id,
        from_location_id,
        to_location_id,
        quantity,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().json(StockTransferResponse {
            message: "stock transfer successfully".to_string(),
        }),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct BookUpdateRequest {
    token: String,
    tag: String,
    nonce: String,
    book_id: u32,
    isbn: String,
    title: String,
    authors: Vec<u32>,
    keywords: Vec<u32>,
    series: Vec<(u32, u32)>,
    suppliers: Vec<u32>,
    publisher: u32,
    price: String,
    catalog: String,
    cover: String,
    is_onstore: bool,
}

#[derive(Debug, Serialize)]
struct BookUpdateResponse {
    message: String,
}

#[post("/admin/book/update")]
pub async fn admin_book_update(
    pool: web::Data<Pool>,
    book_update_request: web::Json<BookUpdateRequest>,
) -> impl Responder {
    let request = book_update_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    let book_id = request.book_id;
    let isbn = &request.isbn;
    let title = &request.title;
    let authors = &request.authors;
    let keywords = &request.keywords;
    let series = &request.series;
    let suppliers = &request.suppliers;
    let publisher = request.publisher;
    let price = &request.price;
    let catalog = &request.catalog;
    let cover = &request.cover;
    let is_onstore = request.is_onstore;

    let mut conn = pool.get_conn().await.unwrap();

    match AdminService::update_book(
        &mut conn,
        token,
        book_id,
        isbn,
        title,
        authors,
        keywords,
        series,
        suppliers,
        publisher,
        price.parse().unwrap(),
        catalog,
        cover,
        is_onstore,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().json(BookUpdateResponse {
            message: "book update successfully".to_string(),
        }),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct BookAddRequest {
    token: String,
    tag: String,
    nonce: String,
    isbn: String,
    title: String,
    authors: Vec<u32>,
    keywords: Vec<u32>,
    series: Vec<(u32, u32)>,
    suppliers: Vec<u32>,
    publisher: u32,
    price: String,
    catalog: String,
    cover: String,
    is_onstore: bool,
}

#[derive(Debug, Serialize)]
struct BookAddResponse {
    message: String,
}

#[post("/admin/book/add")]
pub async fn admin_book_add(
    pool: web::Data<Pool>,
    book_add_request: web::Json<BookAddRequest>,
) -> impl Responder {
    let request = book_add_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    let isbn = &request.isbn;
    let title = &request.title;
    let authors = &request.authors;
    let keywords = &request.keywords;
    let series = &request.series;
    let suppliers = &request.suppliers;
    let publisher = request.publisher;
    let price = &request.price;
    let catalog = &request.catalog;
    let cover = &request.cover;
    let is_onstore = request.is_onstore;

    let mut conn = pool.get_conn().await.unwrap();

    match AdminService::add_book(
        &mut conn,
        token,
        isbn,
        title,
        authors,
        keywords,
        series,
        suppliers,
        publisher,
        price.parse().unwrap(),
        catalog,
        cover,
        is_onstore,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().json(BookAddResponse {
            message: "book add successfully".to_string(),
        }),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct CustomerListRequest {
    token: String,
    tag: String,
    nonce: String,
}

#[derive(Debug, Serialize)]
struct CustomerListItemResponse {
    user_id: u32,
    username: String,
    name: String,
    email: String,
    address: String,
    balance: String,
    credit_level: u32,
}

#[derive(Debug, Serialize)]
struct CustomerListResponse {
    customers: Vec<CustomerListItemResponse>,
}

#[post("/admin/customer/list")]
pub async fn admin_customer_list(
    pool: web::Data<Pool>,
    customer_list_request: web::Json<CustomerListRequest>,
) -> impl Responder {
    let request = customer_list_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };

    let mut conn = pool.get_conn().await.unwrap();

    match AdminService::get_customer_list(&mut conn, token).await {
        Ok(customers) => HttpResponse::Ok().json(CustomerListResponse {
            customers: customers
                .into_iter()
                .map(|customer| CustomerListItemResponse {
                    user_id: customer.id,
                    username: customer.username,
                    name: customer.name,
                    email: customer.email,
                    address: customer.address,
                    balance: customer.account_balance.to_string(),
                    credit_level: customer.credit_level,
                })
                .collect(),
        }),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct CustomerBalanceRequest {
    token: String,
    tag: String,
    nonce: String,
    user_id: u32,
    balance: String,
}

#[derive(Debug, Serialize)]
struct CustomerBalanceResponse {
    message: String,
}

#[post("/admin/customer/balance")]
pub async fn admin_customer_balance(
    pool: web::Data<Pool>,
    customer_balance_request: web::Json<CustomerBalanceRequest>,
) -> impl Responder {
    let request = customer_balance_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    let user_id = request.user_id;
    let balance = request.balance.parse().unwrap();

    let mut conn = pool.get_conn().await.unwrap();

    match AdminService::set_customer_balance(&mut conn, token, user_id, balance).await {
        Ok(_) => HttpResponse::Ok().json(CustomerBalanceResponse {
            message: "customer balance set successfully".to_string(),
        }),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct CustomerCreditRequest {
    token: String,
    tag: String,
    nonce: String,
    user_id: u32,
    credit_level: u32,
}

#[derive(Debug, Serialize)]
struct CustomerCreditResponse {
    message: String,
}

#[post("/admin/customer/credit")]
pub async fn admin_customer_credit(
    pool: web::Data<Pool>,
    customer_credit_request: web::Json<CustomerCreditRequest>,
) -> impl Responder {
    let request = customer_credit_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    let user_id = request.user_id;
    let credit_level = request.credit_level;

    let mut conn = pool.get_conn().await.unwrap();

    match AdminService::set_customer_credit_level(&mut conn, token, user_id, credit_level).await {
        Ok(_) => HttpResponse::Ok().json(CustomerCreditResponse {
            message: "customer credit level set successfully".to_string(),
        }),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct CustomerOrderListRequest {
    token: String,
    tag: String,
    nonce: String,
}

#[derive(Debug, Serialize)]
struct CustomerOrderItemResponse {
    item_id: u32,
    book_id: u32,
    quantity: u32,
    price: String,
}

#[derive(Debug, Serialize)]
struct CustomerOrderListItemResponse {
    order_id: u32,
    user_id: u32,
    total_price: String,
    items: Vec<CustomerOrderItemResponse>,
    date: String,
    original_amount: String,
    total_amount: String,
    shipping_address: String,
    payment_status: String,
    shipping_status: String,
}

#[derive(Debug, Serialize)]
struct CustomerOrderListResponse {
    orders: Vec<CustomerOrderListItemResponse>,
}

#[post("/admin/order/list")]
pub async fn admin_order_list(
    pool: web::Data<Pool>,
    custom_order_list_request: web::Json<CustomerOrderListRequest>,
) -> impl Responder {
    let request = custom_order_list_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };

    let mut conn = pool.get_conn().await.unwrap();

    match AdminService::get_customer_order_list(&mut conn, token).await {
        Ok(orders) => HttpResponse::Ok().json(CustomerOrderListResponse {
            orders: orders
                .into_iter()
                .map(|order| CustomerOrderListItemResponse {
                    order_id: order.id,
                    user_id: order.customer_id,
                    total_price: order.total_amount.to_string(),
                    items: order
                        .items
                        .into_iter()
                        .map(|item| CustomerOrderItemResponse {
                            item_id: item.id,
                            book_id: item.book_id,
                            quantity: item.quantity,
                            price: item.total_price.to_string(),
                        })
                        .collect(),
                    date: order.date.to_string(),
                    original_amount: order.original_amount.to_string(),
                    total_amount: order.total_amount.to_string(),
                    shipping_address: order.shipping_address,
                    payment_status: order.payment_status.to_string(),
                    shipping_status: order.shipping_status.to_string(),
                })
                .collect(),
        }),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct ShipOrderAutoRequest {
    token: String,
    tag: String,
    nonce: String,
    order_id: u32,
}

#[derive(Debug, Serialize)]
struct ShipOrderAutoResponse {
    message: String,
}

#[post("/admin/order/ship/auto")]
pub async fn admin_order_ship_auto(
    pool: web::Data<Pool>,
    ship_order_auto_request: web::Json<ShipOrderAutoRequest>,
) -> impl Responder {
    let request = ship_order_auto_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    let order_id = request.order_id;

    let mut conn = pool.get_conn().await.unwrap();

    match AdminService::ship_order_auto(&mut conn, token, order_id).await {
        Ok(_) => HttpResponse::Ok().json(ShipOrderAutoResponse {
            message: "order ship automatically successfully".to_string(),
        }),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}
