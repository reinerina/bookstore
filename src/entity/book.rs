use crate::entity::series::BookInSeries;
use crate::entity::{Author, Keyword, Publisher, Supplier};
use mysql_common::bigdecimal::BigDecimal;

#[derive(Debug, Default)]
pub struct Book {
    pub id: u32,
    pub isbn: String,
    pub title: String,
    pub authors: Vec<Author>,
    pub keywords: Vec<Keyword>,
    pub publisher: Publisher,
    pub suppliers: Vec<Supplier>,
    pub in_series: Vec<BookInSeries>,
    pub price: BigDecimal,
    pub catalog: String,
    pub cover: String,
    pub is_onstore: bool,
}
