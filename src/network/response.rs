use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub token: String,
    pub tag: String,
    pub nonce: String,
}

#[derive(Deserialize, Debug)]
pub struct LogoutResponse {
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct UserDetailResponse {
    pub username: String,
    pub name: String,
    pub address: String,
    pub email: String,
    pub account_balance: String,
    pub credit_level: u32,
    pub total_purchase: String,
    pub overdraft_limit: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthorListItemResponse {
    pub author_id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct PublisherListItemResponse {
    pub publisher_id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct SupplierListItemResponse {
    pub supplier_id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct SeriesListItemResponse {
    pub series_id: u32,
    pub name: String,
    pub column: u32,
}

#[derive(Debug, Deserialize)]
pub struct BookListItemResponse {
    pub book_id: u32,
    pub isbn: String,
    pub title: String,
    pub authors: Vec<AuthorListItemResponse>,
    pub publisher: PublisherListItemResponse,
    pub suppliers: Vec<SupplierListItemResponse>,
    pub in_series: Vec<SeriesListItemResponse>,
    pub price: String,
    pub keywords: Vec<String>,
    pub cover: String,
    pub is_onstore: bool,
}

#[derive(Debug, Deserialize)]
pub struct BookListResponse {
    pub books: Vec<BookListItemResponse>,
}

#[derive(Debug, Deserialize)]
pub struct AuthorDetailResponse {
    pub author_id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct PublisherDetailResponse {
    pub publisher_id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct SupplierDetailResponse {
    pub supplier_id: u32,
    pub name: String,
    pub telephone: String,
    pub email: String,
    pub address: String,
    pub fax: String,
}

#[derive(Debug, Deserialize)]
pub struct SeriesDetailResponse {
    pub series_id: u32,
    pub name: String,
    pub column: u32,
}

#[derive(Debug, Deserialize)]
pub struct BookDetailResponse {
    pub book_id: u32,
    pub isbn: String,
    pub title: String,
    pub authors: Vec<AuthorDetailResponse>,
    pub publisher: PublisherDetailResponse,
    pub suppliers: Vec<SupplierDetailResponse>,
    pub in_series: Vec<SeriesDetailResponse>,
    pub price: String,
    pub keywords: Vec<String>,
    pub catalog: String,
    pub cover: String,
    pub is_onstore: bool,
}

#[derive(Debug, Deserialize)]
pub struct OrderCreateResponse {
    pub order_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct OrderHistoryItemResponse {
    pub order_id: u32,
    pub discount_percentage: String,
    pub discount_amount: String,
    pub original_price: String,
    pub total_price: String,
    pub order_date: String,
    pub payment_status: String,
    pub shipping_status: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderHistoryResponse {
    pub orders: Vec<OrderHistoryItemResponse>,
}
