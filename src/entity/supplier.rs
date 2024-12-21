use crate::entity::Book;
use mysql_common::bigdecimal::BigDecimal;
use mysql_common::time::PrimitiveDateTime;

#[derive(Debug, Default)]
pub struct SupplierCatalog {
    pub id: u32,
    pub supplier_id: u32,
    pub book: Book,
    pub available_quantity: u32,
    pub price: BigDecimal,
}

#[derive(Debug)]
pub struct SupplierRecord {
    pub id: u32,
    pub supplier_id: u32,
    pub book_id: u32,
    pub price: BigDecimal,
    pub date: PrimitiveDateTime,
    pub quantity_supplied: u32,
}

impl Default for SupplierRecord {
    fn default() -> Self {
        Self {
            id: 0,
            supplier_id: 0,
            book_id: 0,
            price: BigDecimal::default(),
            date: PrimitiveDateTime::MIN,
            quantity_supplied: 0,
        }
    }
}

#[derive(Debug, Default)]
pub struct Supplier {
    pub id: u32,
    pub name: String,
    pub telephone: String,
    pub email: String,
    pub address: String,
    pub fax: String,
}
