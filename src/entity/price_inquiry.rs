use crate::entity::PriceInquiryStatus;
use mysql_common::time::PrimitiveDateTime;

#[derive(Debug)]
pub struct PriceInquiry {
    pub id: u32,
    pub customer_id: u32,
    pub book_title: String,
    pub isbn: String,
    pub date: PrimitiveDateTime,
    pub status: PriceInquiryStatus,
}

impl Default for PriceInquiry {
    fn default() -> Self {
        Self {
            id: 0,
            customer_id: 0,
            book_title: String::default(),
            isbn: String::default(),
            date: PrimitiveDateTime::MIN,
            status: PriceInquiryStatus::default(),
        }
    }
}
