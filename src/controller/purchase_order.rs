use crate::service::PurchaseOrderService;
use crate::utils::Token;
use actix_web::{post, web, HttpResponse, Responder};
use mysql_async::Pool;
use serde::{Deserialize, Serialize};
use std::mem::take;

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
    mut purchase_order_list_request: web::Json<PurchaseOrderListRequest>,
) -> impl Responder {
    let token = &Token {
        token: take(&mut purchase_order_list_request.token),
        tag: take(&mut purchase_order_list_request.tag),
        nonce: take(&mut purchase_order_list_request.nonce),
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
