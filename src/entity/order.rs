use crate::entity::{OrderPaymentStatus, OrderShippingStatus};
use mysql_common::bigdecimal::BigDecimal;
use mysql_common::time::PrimitiveDateTime;

#[derive(Debug, Default)]
pub struct OrderItem {
    pub id: u32,
    pub order_id: u32,
    pub book_id: u32,
    pub quantity: u32,
    pub total_price: BigDecimal,
}

#[derive(Debug)]
pub struct Order {
    pub id: u32,
    pub customer_id: u32,
    pub items: Vec<OrderItem>,
    pub date: PrimitiveDateTime,
    pub discount_percentage: BigDecimal,
    pub discount_amount: BigDecimal,
    pub original_amount: BigDecimal,
    pub total_amount: BigDecimal,
    pub shipping_address: String,
    pub payment_status: OrderPaymentStatus,
    pub shipping_status: OrderShippingStatus,
}

impl Default for Order {
    fn default() -> Self {
        Self {
            id: 0,
            customer_id: 0,
            items: Vec::new(),
            date: PrimitiveDateTime::MIN,
            discount_percentage: BigDecimal::default(),
            discount_amount: BigDecimal::default(),
            original_amount: BigDecimal::default(),
            total_amount: BigDecimal::default(),
            shipping_address: String::default(),
            payment_status: OrderPaymentStatus::default(),
            shipping_status: OrderShippingStatus::default(),
        }
    }
}
