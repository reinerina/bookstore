use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct AdminLoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Default)]
pub struct AdminRegisterRequest {
    pub username: String,
    pub password: String,
    pub role: String,
    pub token: String,
    pub tag: String,
    pub nonce: String,
}

#[derive(Debug, Serialize, Default)]
pub struct AdminDetailRequest {
    pub token: String,
    pub tag: String,
    pub nonce: String,
}

#[derive(Serialize, Debug, Default)]
pub struct BookListRequest;

#[derive(Serialize, Debug, Default)]
pub struct BookDetailRequest {
    pub token: String,
    pub tag: String,
    pub nonce: String,
    pub book_id: u32,
}

#[derive(Serialize, Debug, Default)]
pub struct LocationListRequest {
    pub token: String,
    pub tag: String,
    pub nonce: String,
}

#[derive(Serialize, Debug, Default)]
pub struct StockChangeRequest {
    pub token: String,
    pub tag: String,
    pub nonce: String,
    pub book_id: u32,
    pub location_id: u32,
    pub quantity: i32,
}

#[derive(Serialize, Debug, Default)]
pub struct StockTransferRequest {
    pub token: String,
    pub tag: String,
    pub nonce: String,
    pub book_id: u32,
    pub from_location_id: u32,
    pub to_location_id: u32,
    pub quantity: u32,
}

#[derive(Serialize, Debug, Default)]
pub struct AuthorListRequest;

#[derive(Serialize, Debug, Default)]
pub struct KeywordListRequest;

#[derive(Serialize, Debug, Default)]
pub struct PublisherListRequest;

#[derive(Serialize, Debug, Default)]
pub struct SupplierListRequest;

#[derive(Serialize, Debug, Default)]
pub struct SeriesListRequest;

#[derive(Serialize, Debug, Default)]
pub struct BookUpdateRequest {
    pub token: String,
    pub tag: String,
    pub nonce: String,
    pub book_id: u32,
    pub isbn: String,
    pub title: String,
    pub authors: Vec<u32>,
    pub keywords: Vec<u32>,
    pub series: Vec<(u32, u32)>,
    pub suppliers: Vec<u32>,
    pub publisher: u32,
    pub price: String,
    pub catalog: String,
    pub cover: String,
    pub is_onstore: bool,
}

#[derive(Serialize, Debug, Default)]
pub struct BookAddRequest {
    pub token: String,
    pub tag: String,
    pub nonce: String,
    pub isbn: String,
    pub title: String,
    pub authors: Vec<u32>,
    pub keywords: Vec<u32>,
    pub series: Vec<(u32, u32)>,
    pub suppliers: Vec<u32>,
    pub publisher: u32,
    pub price: String,
    pub catalog: String,
    pub cover: String,
    pub is_onstore: bool,
}

#[derive(Serialize, Debug, Default)]
pub struct CustomerListRequest {
    pub token: String,
    pub tag: String,
    pub nonce: String,
}

#[derive(Serialize, Debug, Default)]
pub struct CustomerBalanceRequest {
    pub token: String,
    pub tag: String,
    pub nonce: String,
    pub user_id: u32,
    pub balance: String,
}

#[derive(Serialize, Debug, Default)]
pub struct CustomerCreditRequest {
    pub token: String,
    pub tag: String,
    pub nonce: String,
    pub user_id: u32,
    pub credit_level: u32,
}