use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub token: String,
    pub tag: String,
    pub nonce: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterResponse {
    pub token: String,
    pub tag: String,
    pub nonce: String,
}

#[derive(Deserialize, Debug)]
pub struct LogoutResponse {
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct UserUpdateResponse {
    pub token: String,
    pub tag: String,
    pub nonce: String,
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
pub struct OrderItemResponse {
    pub book_id: u32,
    pub title: String,
    pub publisher: PublisherDetailResponse,
    pub cover: String,
    pub price: String,
    pub quantity: u32,
    pub total_price: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderDetailResponse {
    pub order_id: u32,
    pub discount_percentage: String,
    pub discount_amount: String,
    pub original_price: String,
    pub total_price: String,
    pub order_date: String,
    pub payment_status: String,
    pub shipping_status: String,
    pub shipping_address: String,
    pub items: Vec<OrderItemResponse>,
}

#[derive(Debug, Deserialize)]
pub struct OrderPaymentResponse {
    pub message: String,
}
