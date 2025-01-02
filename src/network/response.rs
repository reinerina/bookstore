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

#[derive(Debug, Deserialize)]
pub struct PurchaseOrderCreateResponse {
    pub purchase_order_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct PurchaseOrderListItemResponse {
    pub purchase_order_id: u32,
    pub order_date: String,
    pub expected_delivery_date: String,
    pub status: String,
    pub total_price: String,
}

#[derive(Debug, Deserialize)]
pub struct PurchaseOrderListResponse {
    pub purchase_orders: Vec<PurchaseOrderListItemResponse>,
}

#[derive(Debug, Deserialize)]
pub struct PurchaseOrderDetailItemResponse {
    pub supplier_catalog_id: u32,
    pub book_id: u32,
    pub title: String,
    pub isbn: String,
    pub supplier_id: u32,
    pub supplier_name: String,
    pub publisher_id: u32,
    pub publisher_name: String,
    pub quantity: u32,
    pub total_price: String,
}

#[derive(Debug, Deserialize)]
pub struct PurchaseOrderDetailResponse {
    pub purchase_order_id: u32,
    pub order_date: String,
    pub expected_delivery_date: String,
    pub status: String,
    pub total_price: String,
    pub items: Vec<PurchaseOrderDetailItemResponse>,
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

#[derive(Debug, Deserialize)]
pub struct BookTitleSearchItemResponse {
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

impl Into<BookListItemResponse> for BookTitleSearchItemResponse {
    fn into(self) -> BookListItemResponse {
        BookListItemResponse {
            book_id: self.book_id,
            isbn: self.isbn,
            title: self.title,
            authors: self.authors,
            publisher: self.publisher,
            suppliers: self.suppliers,
            in_series: self.in_series,
            price: self.price,
            keywords: self.keywords,
            cover: self.cover,
            is_onstore: self.is_onstore,
        }
    }
}

impl Into<BookKeywordsSearchItemResponse> for BookTitleSearchItemResponse {
    fn into(self) -> BookKeywordsSearchItemResponse {
        BookKeywordsSearchItemResponse {
            book_id: self.book_id,
            isbn: self.isbn,
            title: self.title,
            authors: self.authors,
            publisher: self.publisher,
            suppliers: self.suppliers,
            in_series: self.in_series,
            price: self.price,
            keywords: self.keywords,
            cover: self.cover,
            is_onstore: self.is_onstore,
        }
    }
}

impl Into<BookAuthorsSearchItemResponse> for BookTitleSearchItemResponse {
    fn into(self) -> BookAuthorsSearchItemResponse {
        BookAuthorsSearchItemResponse {
            book_id: self.book_id,
            isbn: self.isbn,
            title: self.title,
            authors: self.authors,
            publisher: self.publisher,
            suppliers: self.suppliers,
            in_series: self.in_series,
            price: self.price,
            keywords: self.keywords,
            cover: self.cover,
            is_onstore: self.is_onstore,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BookTitleSearchResponse {
    pub books: Vec<BookTitleSearchItemResponse>,
}

impl Into<BookListResponse> for BookTitleSearchResponse {
    fn into(self) -> BookListResponse {
        BookListResponse {
            books: self.books.into_iter().map(|b| b.into()).collect(),
        }
    }
}

impl Into<BookAuthorsSearchResponse> for BookTitleSearchResponse {
    fn into(self) -> BookAuthorsSearchResponse {
        BookAuthorsSearchResponse {
            books: self.books.into_iter().map(|b| b.into()).collect(),
        }
    }
}

impl Into<BookKeywordsSearchResponse> for BookTitleSearchResponse {
    fn into(self) -> BookKeywordsSearchResponse {
        BookKeywordsSearchResponse {
            books: self.books.into_iter().map(|b| b.into()).collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BookKeywordsSearchItemResponse {
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

impl Into<BookTitleSearchItemResponse> for BookKeywordsSearchItemResponse {
    fn into(self) -> BookTitleSearchItemResponse {
        BookTitleSearchItemResponse {
            book_id: self.book_id,
            isbn: self.isbn,
            title: self.title,
            authors: self.authors,
            publisher: self.publisher,
            suppliers: self.suppliers,
            in_series: self.in_series,
            price: self.price,
            keywords: self.keywords,
            cover: self.cover,
            is_onstore: self.is_onstore,
        }
    }
}

impl Into<BookListItemResponse> for BookKeywordsSearchItemResponse {
    fn into(self) -> BookListItemResponse {
        BookListItemResponse {
            book_id: self.book_id,
            isbn: self.isbn,
            title: self.title,
            authors: self.authors,
            publisher: self.publisher,
            suppliers: self.suppliers,
            in_series: self.in_series,
            price: self.price,
            keywords: self.keywords,
            cover: self.cover,
            is_onstore: self.is_onstore,
        }
    }
}

impl Into<BookAuthorsSearchItemResponse> for BookKeywordsSearchItemResponse {
    fn into(self) -> BookAuthorsSearchItemResponse {
        BookAuthorsSearchItemResponse {
            book_id: self.book_id,
            isbn: self.isbn,
            title: self.title,
            authors: self.authors,
            publisher: self.publisher,
            suppliers: self.suppliers,
            in_series: self.in_series,
            price: self.price,
            keywords: self.keywords,
            cover: self.cover,
            is_onstore: self.is_onstore,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BookKeywordsSearchResponse {
    pub books: Vec<BookKeywordsSearchItemResponse>,
}

impl Into<BookTitleSearchResponse> for BookKeywordsSearchResponse {
    fn into(self) -> BookTitleSearchResponse {
        BookTitleSearchResponse {
            books: self.books.into_iter().map(|b| b.into()).collect(),
        }
    }
}

impl Into<BookListResponse> for BookKeywordsSearchResponse {
    fn into(self) -> BookListResponse {
        BookListResponse {
            books: self.books.into_iter().map(|b| b.into()).collect(),
        }
    }
}

impl Into<BookAuthorsSearchResponse> for BookKeywordsSearchResponse {
    fn into(self) -> BookAuthorsSearchResponse {
        BookAuthorsSearchResponse {
            books: self.books.into_iter().map(|b| b.into()).collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BookAuthorsSearchItemResponse {
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

impl Into<BookTitleSearchItemResponse> for BookAuthorsSearchItemResponse {
    fn into(self) -> BookTitleSearchItemResponse {
        BookTitleSearchItemResponse {
            book_id: self.book_id,
            isbn: self.isbn,
            title: self.title,
            authors: self.authors,
            publisher: self.publisher,
            suppliers: self.suppliers,
            in_series: self.in_series,
            price: self.price,
            keywords: self.keywords,
            cover: self.cover,
            is_onstore: self.is_onstore,
        }
    }
}

impl Into<BookKeywordsSearchItemResponse> for BookAuthorsSearchItemResponse {
    fn into(self) -> BookKeywordsSearchItemResponse {
        BookKeywordsSearchItemResponse {
            book_id: self.book_id,
            isbn: self.isbn,
            title: self.title,
            authors: self.authors,
            publisher: self.publisher,
            suppliers: self.suppliers,
            in_series: self.in_series,
            price: self.price,
            keywords: self.keywords,
            cover: self.cover,
            is_onstore: self.is_onstore,
        }
    }
}

impl Into<BookListItemResponse> for BookAuthorsSearchItemResponse {
    fn into(self) -> BookListItemResponse {
        BookListItemResponse {
            book_id: self.book_id,
            isbn: self.isbn,
            title: self.title,
            authors: self.authors,
            publisher: self.publisher,
            suppliers: self.suppliers,
            in_series: self.in_series,
            price: self.price,
            keywords: self.keywords,
            cover: self.cover,
            is_onstore: self.is_onstore,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BookAuthorsSearchResponse {
    pub books: Vec<BookAuthorsSearchItemResponse>,
}

impl Into<BookTitleSearchResponse> for BookAuthorsSearchResponse {
    fn into(self) -> BookTitleSearchResponse {
        BookTitleSearchResponse {
            books: self.books.into_iter().map(|b| b.into()).collect(),
        }
    }
}

impl Into<BookKeywordsSearchResponse> for BookAuthorsSearchResponse {
    fn into(self) -> BookKeywordsSearchResponse {
        BookKeywordsSearchResponse {
            books: self.books.into_iter().map(|b| b.into()).collect(),
        }
    }
}

impl Into<BookListResponse> for BookAuthorsSearchResponse {
    fn into(self) -> BookListResponse {
        BookListResponse {
            books: self.books.into_iter().map(|b| b.into()).collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserSearchItemResponse {
    pub user_id: u32,
    pub username: String,
    pub name: String,
    pub email: String,
    pub address: String,
    pub balance: String,
    pub credit_level: u32,
}

#[derive(Debug, Deserialize)]
pub struct UserSearchResponse {
    pub users: Vec<UserSearchItemResponse>,
}
