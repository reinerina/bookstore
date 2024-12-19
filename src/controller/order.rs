use crate::service::OrderService;
use crate::utils::Token;
use actix_web::{post, web, HttpResponse, Responder};
use mysql_async::Pool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct OrderCreateRequest {
    token: String,
    tag: String,
    nonce: String,
    items: Vec<(u32, u32)>,
}

#[derive(Debug, Serialize)]
struct OrderCreateResponse {
    order_id: u32,
}

#[post("/order/create")]
pub async fn order_create(
    pool: web::Data<Pool>,
    order_create_request: web::Json<OrderCreateRequest>,
) -> impl Responder {
    let request = order_create_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    let items = &request.items;

    match pool.get_conn().await {
        Ok(mut conn) => match OrderService::create_order(&mut conn, token, items).await {
            Ok(order) => HttpResponse::Ok().json(OrderCreateResponse { order_id: order }),
            Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct OrderHistoryRequest {
    token: String,
    tag: String,
    nonce: String,
}

#[derive(Debug, Serialize)]
struct OrderHistoryItemResponse {
    order_id: u32,
    discount_percentage: String,
    discount_amount: String,
    original_price: String,
    total_price: String,
    order_date: String,
    payment_status: String,
    shipping_status: String,
}

#[derive(Debug, Serialize)]
struct OrderHistoryResponse {
    orders: Vec<OrderHistoryItemResponse>,
}

#[post("/order/history")]
pub async fn order_history(
    pool: web::Data<Pool>,
    order_history_request: web::Json<OrderHistoryRequest>,
) -> impl Responder {
    let request = order_history_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };

    match pool.get_conn().await {
        Ok(mut conn) => match OrderService::get_order_list(&mut conn, token).await {
            Ok(orders) => HttpResponse::Ok().json(OrderHistoryResponse {
                orders: orders
                    .iter()
                    .map(|order| OrderHistoryItemResponse {
                        order_id: order.id,
                        discount_percentage: order.discount_percentage.with_scale(1).to_string(),
                        discount_amount: order.discount_amount.with_scale(2).to_string(),
                        original_price: order.original_amount.with_scale(2).to_string(),
                        total_price: order.total_amount.with_scale(2).to_string(),
                        order_date: order.date.to_string(),
                        payment_status: order.payment_status.to_string(),
                        shipping_status: order.shipping_status.to_string(),
                    })
                    .collect(),
            }),
            Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct OrderDetailRequest {
    token: String,
    tag: String,
    nonce: String,
}

#[derive(Debug, Serialize)]
struct PublisherDetailResponse {
    publisher_id: u32,
    name: String,
}

#[derive(Debug, Serialize)]
struct OrderItemResponse {
    book_id: u32,
    title: String,
    publisher: PublisherDetailResponse,
    cover: String,
    price: String,
    quantity: u32,
    total_price: String,
}

#[derive(Debug, Serialize)]
struct OrderDetailResponse {
    order_id: u32,
    discount_percentage: String,
    discount_amount: String,
    original_price: String,
    total_price: String,
    order_date: String,
    payment_status: String,
    shipping_status: String,
    shipping_address: String,
    items: Vec<OrderItemResponse>,
}

#[post("/order/{id}/detail")]
pub async fn order_detail(
    pool: web::Data<Pool>,
    order_detail_request: web::Json<OrderDetailRequest>,
    id: web::Path<(u32,)>,
) -> impl Responder {
    let request = order_detail_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    match pool.get_conn().await {
        Ok(mut conn) => {
            match OrderService::get_order_detail(&mut conn, token, id.into_inner().0).await {
                Ok((order, book)) => {
                    let mut order_items = Vec::new();
                    for item in book.into_iter().zip(order.items.iter()) {
                        let (book, order_item) = item;
                        order_items.push(OrderItemResponse {
                            book_id: book.id,
                            title: book.title,
                            publisher: PublisherDetailResponse {
                                publisher_id: book.publisher.id,
                                name: book.publisher.name,
                            },
                            cover: book.cover,
                            price: book.price.with_scale(2).to_string(),
                            quantity: order_item.quantity,
                            total_price: order_item.total_price.with_scale(2).to_string(),
                        });
                    }
                    HttpResponse::Ok().json(OrderDetailResponse {
                        order_id: order.id,
                        discount_percentage: order.discount_percentage.with_scale(1).to_string(),
                        discount_amount: order.discount_amount.with_scale(2).to_string(),
                        original_price: order.original_amount.with_scale(2).to_string(),
                        total_price: order.total_amount.with_scale(2).to_string(),
                        order_date: order.date.to_string(),
                        payment_status: order.payment_status.to_string(),
                        shipping_status: order.shipping_status.to_string(),
                        shipping_address: order.shipping_address,
                        items: order_items,
                    })
                }
                Err(e) => HttpResponse::BadRequest().json(e.to_string()),
            }
        }
        Err(e) => HttpResponse::BadGateway().json(e.to_string()),
    }
}
