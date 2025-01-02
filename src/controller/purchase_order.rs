use crate::service::PurchaseOrderService;
use crate::utils::Token;
use actix_web::{post, web, HttpResponse, Responder};
use mysql_async::Pool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct PurchaseOrderListRequest {
    token: String,
    tag: String,
    nonce: String,
}

#[derive(Debug, Serialize)]
struct PurchaseOrderListItemResponse {
    purchase_order_id: u32,
    order_date: String,
    expected_delivery_date: String,
    status: String,
    total_price: String,
}

#[derive(Debug, Serialize)]
struct PurchaseOrderListResponse {
    purchase_orders: Vec<PurchaseOrderListItemResponse>,
}

#[post("/purchase_order/list")]
pub async fn purchase_order_list(
    pool: web::Data<Pool>,
    purchase_order_list_request: web::Json<PurchaseOrderListRequest>,
) -> impl Responder {
    let request = purchase_order_list_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    match pool.get_conn().await {
        Ok(mut conn) => match PurchaseOrderService::get_purchase_order_list(&mut conn, token).await
        {
            Ok(purchase_orders) => HttpResponse::Ok().json(PurchaseOrderListResponse {
                purchase_orders: purchase_orders
                    .iter()
                    .map(|purchase_order| PurchaseOrderListItemResponse {
                        purchase_order_id: purchase_order.id,
                        order_date: purchase_order.order_date.to_string(),
                        expected_delivery_date: purchase_order.expected_delivery_date.to_string(),
                        status: purchase_order.status.to_string(),
                        total_price: purchase_order.total_amount.to_string(),
                    })
                    .collect(),
            }),
            Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct PurchaseOrderDetailRequest {
    token: String,
    tag: String,
    nonce: String,
    purchase_order_id: u32,
}

#[derive(Debug, Serialize)]
struct PurchaseOrderDetailItemResponse {
    supplier_catalog_id: u32,
    book_id: u32,
    title: String,
    isbn: String,
    supplier_id: u32,
    supplier_name: String,
    publisher_id: u32,
    publisher_name: String,
    quantity: u32,
    total_price: String,
}

#[derive(Debug, Serialize)]
struct PurchaseOrderDetailResponse {
    purchase_order_id: u32,
    order_date: String,
    expected_delivery_date: String,
    status: String,
    total_price: String,
    items: Vec<PurchaseOrderDetailItemResponse>,
}

#[post("/purchase_order/detail")]
pub async fn purchase_order_detail(
    pool: web::Data<Pool>,
    purchase_order_detail_request: web::Json<PurchaseOrderDetailRequest>,
) -> impl Responder {
    let request = purchase_order_detail_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    match pool.get_conn().await {
        Ok(mut conn) => match PurchaseOrderService::get_purchase_order_detail(
            &mut conn,
            token,
            request.purchase_order_id,
        )
        .await
        {
            Ok((purchase_order, catalogs)) => {
                HttpResponse::Ok().json(PurchaseOrderDetailResponse {
                    purchase_order_id: purchase_order.id,
                    order_date: purchase_order.order_date.to_string(),
                    expected_delivery_date: purchase_order.expected_delivery_date.to_string(),
                    status: purchase_order.status.to_string(),
                    total_price: purchase_order.total_amount.to_string(),
                    items: purchase_order
                        .items
                        .into_iter()
                        .zip(catalogs.into_iter())
                        .map(|(item, catalogs)| PurchaseOrderDetailItemResponse {
                            supplier_catalog_id: item.supplier_catalog_id,
                            book_id: catalogs.book.id,
                            title: catalogs.book.title,
                            isbn: catalogs.book.isbn,
                            supplier_id: catalogs.supplier.id,
                            supplier_name: catalogs.supplier.name,
                            publisher_id: catalogs.book.publisher.id,
                            publisher_name: catalogs.book.publisher.name,
                            quantity: item.quantity,
                            total_price: item.total_price.to_string(),
                        })
                        .collect(),
                })
            }
            Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct PurchaseOrderCreateRequest {
    token: String,
    tag: String,
    nonce: String,
    shortage_id: u32,
}

#[derive(Debug, Serialize)]
struct PurchaseOrderCreateResponse {
    purchase_order_id: u32,
}

#[post("/purchase_order/create")]
pub async fn purchase_order_create(
    pool: web::Data<Pool>,
    purchase_order_create_request: web::Json<PurchaseOrderCreateRequest>,
) -> impl Responder {
    let request = purchase_order_create_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    match pool.get_conn().await {
        Ok(mut conn) => {
            match PurchaseOrderService::create_purchase_order(&mut conn, token, request.shortage_id)
                .await
            {
                Ok(purchase_order_id) => {
                    HttpResponse::Ok().json(PurchaseOrderCreateResponse { purchase_order_id })
                }
                Err(e) => HttpResponse::BadRequest().json(e.to_string()),
            }
        }
        Err(e) => HttpResponse::BadGateway().json(e.to_string()),
    }
}
