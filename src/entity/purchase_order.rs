use crate::entity::PurchaseOrderStatus;
use mysql_common::bigdecimal::BigDecimal;
use mysql_common::time::PrimitiveDateTime;

#[derive(Debug, Default)]
pub struct PurchaseOrderItem {
    pub id: u32,
    pub purchase_order_id: u32,
    pub supplier_catalog_id: u32,
    pub quantity: u32,
    pub total_price: BigDecimal,
}

#[derive(Debug)]
pub struct PurchaseOrder {
    pub id: u32,
    pub order_date: PrimitiveDateTime,
    pub expected_delivery_date: PrimitiveDateTime,
    pub status: PurchaseOrderStatus,
    pub items: Vec<PurchaseOrderItem>,
    pub total_amount: BigDecimal,
}

impl Default for PurchaseOrder {
    fn default() -> Self {
        Self {
            id: 0,
            order_date: PrimitiveDateTime::MIN,
            expected_delivery_date: PrimitiveDateTime::MIN,
            status: PurchaseOrderStatus::default(),
            items: Vec::new(),
            total_amount: BigDecimal::default(),
        }
    }
}
