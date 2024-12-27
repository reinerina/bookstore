use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AdminLoginResponse {
    pub token: String,
    pub tag: String,
    pub nonce: String,
}

#[derive(Debug, Deserialize)]
pub struct AdminRegisterResponse {
    pub admin_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct AdminDetailResponse {
    pub admin_id: u32,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthorListItemResponse {
    pub author_id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct KeywordListItemResponse {
    pub keyword_id: u32,
    pub keyword: String,
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
    pub keywords: Vec<KeywordListItemResponse>,
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
pub struct KeywordDetailResponse {
    pub keyword_id: u32,
    pub keyword: String,
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
pub struct LocationDetailResponse {
    pub id: u32,
    pub book_id: u32,
    pub description: String,
    pub quantity: u32,
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
    pub keywords: Vec<KeywordDetailResponse>,
    pub locations: Vec<LocationDetailResponse>,
    pub catalog: String,
    pub cover: String,
    pub is_onstore: bool,
}

#[derive(Debug, Deserialize)]
pub struct LocationListItemResponse {
    pub id: u32,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct LocationListResponse {
    pub locations: Vec<LocationListItemResponse>,
}

#[derive(Debug, Deserialize)]
pub struct StockChangeResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct StockTransferResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthorListResponse {
    pub authors: Vec<AuthorListItemResponse>,
}

#[derive(Debug, Deserialize)]
pub struct KeywordListResponse {
    pub keywords: Vec<KeywordListItemResponse>,
}

#[derive(Debug, Deserialize)]
pub struct PublisherListResponse {
    pub publishers: Vec<PublisherListItemResponse>,
}

#[derive(Debug, Deserialize)]
pub struct SupplierListResponse {
    pub suppliers: Vec<SupplierListItemResponse>,
}

#[derive(Debug, Deserialize)]
pub struct SeriesListResponse {
    pub series: Vec<SeriesListItemResponse>,
}

#[derive(Debug, Deserialize)]
pub struct BookUpdateResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct BookAddResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct CustomerListItemResponse {
    pub user_id: u32,
    pub username: String,
    pub name: String,
    pub email: String,
    pub address: String,
    pub balance: String,
    pub credit_level: u32,
}

#[derive(Debug, Deserialize)]
pub struct CustomerListResponse {
    pub customers: Vec<CustomerListItemResponse>,
}

#[derive(Debug, Deserialize)]
pub struct CustomerBalanceResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct CustomerCreditResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct CustomerOrderItemResponse {
    pub item_id: u32,
    pub book_id: u32,
    pub quantity: u32,
    pub price: String,
}

#[derive(Debug, Deserialize)]
pub struct CustomerOrderListItemResponse {
    pub order_id: u32,
    pub user_id: u32,
    pub total_price: String,
    pub items: Vec<CustomerOrderItemResponse>,
    pub date: String,
    pub original_amount: String,
    pub total_amount: String,
    pub shipping_address: String,
    pub payment_status: String,
    pub shipping_status: String,
}

#[derive(Debug, Deserialize)]
pub struct CustomerOrderListResponse {
    pub orders: Vec<CustomerOrderListItemResponse>,
}

#[derive(Debug, Deserialize)]
pub struct ShipOrderAutoResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ShortageListItemResponse {
    pub shortage_id: u32,
    pub registration_date: String,
    pub is_resolved: bool,
}

#[derive(Debug, Deserialize)]
pub struct ShortageListResponse {
    pub shortages: Vec<ShortageListItemResponse>,
}

#[derive(Debug, Deserialize)]
pub struct ShortageDetailResponse {
    pub shortage_id: u32,
    pub registration_date: String,
    pub is_resolved: bool,
    pub items: Vec<(u32, u32, u32, u32, u32)>,
}

#[derive(Debug, Deserialize)]
pub struct ShortageCreateResponse {
    pub shortage_id: u32,
}
