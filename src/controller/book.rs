use crate::service::BookService;
use crate::utils::Token;
use actix_web::{post, web, HttpResponse, Responder};
use mysql_async::Pool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct AuthorDetailResponse {
    author_id: u32,
    name: String,
}

#[derive(Debug, Serialize)]
struct KeywordDetailResponse {
    keyword_id: u32,
    keyword: String,
}

#[derive(Debug, Serialize)]
struct PublisherDetailResponse {
    publisher_id: u32,
    name: String,
}

#[derive(Debug, Serialize)]
struct SupplierDetailResponse {
    supplier_id: u32,
    name: String,
    telephone: String,
    email: String,
    address: String,
    fax: String,
}

#[derive(Debug, Serialize)]
struct SeriesDetailResponse {
    series_id: u32,
    name: String,
    column: u32,
}

#[derive(Debug, Serialize)]
struct BookDetailResponse {
    book_id: u32,
    isbn: String,
    title: String,
    authors: Vec<AuthorDetailResponse>,
    publisher: PublisherDetailResponse,
    suppliers: Vec<SupplierDetailResponse>,
    in_series: Vec<SeriesDetailResponse>,
    price: String,
    keywords: Vec<KeywordDetailResponse>,
    catalog: String,
    cover: String,
    is_onstore: bool,
}

#[post("/book/{id}/detail")]
pub async fn book_detail(pool: web::Data<Pool>, id: web::Path<(u32,)>) -> impl Responder {
    let mut conn = pool.get_conn().await.unwrap();
    match BookService::get_book_detail(&mut conn, id.into_inner().0).await {
        Ok(book) => HttpResponse::Ok().json(BookDetailResponse {
            book_id: book.id,
            isbn: book.isbn,
            title: book.title,
            authors: book
                .authors
                .into_iter()
                .map(|author| AuthorDetailResponse {
                    author_id: author.id,
                    name: author.name,
                })
                .collect(),
            publisher: PublisherDetailResponse {
                publisher_id: book.publisher.id,
                name: book.publisher.name,
            },
            suppliers: book
                .suppliers
                .into_iter()
                .map(|supplier| SupplierDetailResponse {
                    supplier_id: supplier.id,
                    name: supplier.name,
                    telephone: supplier.telephone,
                    email: supplier.email,
                    address: supplier.address,
                    fax: supplier.fax,
                })
                .collect(),
            in_series: book
                .in_series
                .into_iter()
                .map(|series| SeriesDetailResponse {
                    series_id: series.series_id,
                    name: series.title,
                    column: series.column,
                })
                .collect(),
            price: book.price.to_string(),
            keywords: book
                .keywords
                .into_iter()
                .map(|keyword| KeywordDetailResponse {
                    keyword_id: keyword.id,
                    keyword: keyword.keyword,
                })
                .collect(),
            catalog: book.catalog,
            cover: book.cover,
            is_onstore: book.is_onstore,
        }),
        Err(e) => HttpResponse::BadGateway().body(e.to_string()),
    }
}

#[derive(Debug, Serialize)]
struct AuthorListItemResponse {
    author_id: u32,
    name: String,
}

#[derive(Debug, Serialize)]
struct PublisherListItemResponse {
    publisher_id: u32,
    name: String,
}

#[derive(Debug, Serialize)]
struct SupplierListItemResponse {
    supplier_id: u32,
    name: String,
}

#[derive(Debug, Serialize)]
struct SeriesListItemResponse {
    series_id: u32,
    name: String,
    column: u32,
}

#[derive(Debug, Serialize)]
struct BookListItemResponse {
    book_id: u32,
    isbn: String,
    title: String,
    authors: Vec<AuthorListItemResponse>,
    publisher: PublisherListItemResponse,
    suppliers: Vec<SupplierListItemResponse>,
    in_series: Vec<SeriesListItemResponse>,
    price: String,
    keywords: Vec<KeywordListItemResponse>,
    cover: String,
    is_onstore: bool,
}

#[derive(Debug, Serialize)]
struct BookListResponse {
    books: Vec<BookListItemResponse>,
}

#[post("/book/list")]
pub async fn book_list(pool: web::Data<Pool>) -> impl Responder {
    match pool.get_conn().await {
        Ok(mut conn) => match BookService::get_book_list(&mut conn).await {
            Ok(books) => {
                let books = books
                    .into_iter()
                    .map(|book| BookListItemResponse {
                        book_id: book.id,
                        isbn: book.isbn,
                        title: book.title,
                        authors: book
                            .authors
                            .iter()
                            .map(|author| AuthorListItemResponse {
                                author_id: author.id,
                                name: author.name.clone(),
                            })
                            .collect(),
                        publisher: PublisherListItemResponse {
                            publisher_id: book.publisher.id,
                            name: book.publisher.name,
                        },
                        suppliers: book
                            .suppliers
                            .into_iter()
                            .map(|supplier| SupplierListItemResponse {
                                supplier_id: supplier.id,
                                name: supplier.name,
                            })
                            .collect(),
                        in_series: book
                            .in_series
                            .into_iter()
                            .map(|series| SeriesListItemResponse {
                                series_id: series.series_id,
                                name: series.title,
                                column: series.column,
                            })
                            .collect(),
                        price: book.price.to_string(),
                        keywords: book
                            .keywords
                            .into_iter()
                            .map(|keyword| KeywordListItemResponse {
                                keyword_id: keyword.id,
                                keyword: keyword.keyword,
                            })
                            .collect(),
                        cover: book.cover,
                        is_onstore: book.is_onstore,
                    })
                    .collect();
                HttpResponse::Ok().json(BookListResponse { books })
            }
            Err(e) => HttpResponse::BadGateway().body(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().body(e.to_string()),
    }
}

#[derive(Debug, Serialize)]
struct KeywordListItemResponse {
    keyword_id: u32,
    keyword: String,
}

#[derive(Debug, Serialize)]
struct KeywordListResponse {
    keywords: Vec<KeywordListItemResponse>,
}

#[post("/book/keyword/list")]
pub async fn keyword_list(pool: web::Data<Pool>) -> impl Responder {
    match pool.get_conn().await {
        Ok(mut conn) => match BookService::get_keyword_list(&mut conn).await {
            Ok(keywords) => HttpResponse::Ok().json(KeywordListResponse {
                keywords: keywords
                    .into_iter()
                    .map(|keyword| KeywordListItemResponse {
                        keyword_id: keyword.id,
                        keyword: keyword.keyword,
                    })
                    .collect(),
            }),
            Err(e) => HttpResponse::BadGateway().body(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().body(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct KeywordAddRequest {
    keyword: String,
    token: String,
    tag: String,
    nonce: String,
}

#[derive(Debug, Serialize)]
struct KeywordAddResponse {
    keyword_id: u32,
}

#[post("/book/keyword/add")]
pub async fn keyword_add(
    pool: web::Data<Pool>,
    keyword_add_request: web::Json<KeywordAddRequest>,
) -> impl Responder {
    let request = keyword_add_request.into_inner();
    let token = &Token {
        token: request.token,
        tag: request.tag,
        nonce: request.nonce,
    };
    let keyword = &request.keyword;
    match pool.get_conn().await {
        Ok(mut conn) => match BookService::add_keyword(&mut conn, token, keyword).await {
            Ok(keyword_id) => HttpResponse::Ok().json(KeywordAddResponse { keyword_id }),
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().body(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct BookTitleSearchRequest {
    title: String,
}

#[derive(Debug, Serialize)]
struct BookTitleSearchItemResponse {
    book_id: u32,
    isbn: String,
    title: String,
    authors: Vec<AuthorListItemResponse>,
    publisher: PublisherListItemResponse,
    suppliers: Vec<SupplierListItemResponse>,
    in_series: Vec<SeriesListItemResponse>,
    price: String,
    keywords: Vec<KeywordListItemResponse>,
    cover: String,
    is_onstore: bool,
}

#[derive(Debug, Serialize)]
struct BookTitleSearchResponse {
    books: Vec<BookTitleSearchItemResponse>,
}

#[post("/book/search/title")]
pub async fn book_title_search(
    pool: web::Data<Pool>,
    title: web::Json<BookTitleSearchRequest>,
) -> impl Responder {
    let request = title.into_inner();
    let title = &request.title;
    match pool.get_conn().await {
        Ok(mut conn) => match BookService::search_by_title_natural(&mut conn, title).await {
            Ok(books) => {
                let books = books
                    .into_iter()
                    .map(|book| BookTitleSearchItemResponse {
                        book_id: book.id,
                        isbn: book.isbn,
                        title: book.title,
                        authors: book
                            .authors
                            .into_iter()
                            .map(|author| AuthorListItemResponse {
                                author_id: author.id,
                                name: author.name,
                            })
                            .collect(),
                        publisher: PublisherListItemResponse {
                            publisher_id: book.publisher.id,
                            name: book.publisher.name,
                        },
                        suppliers: book
                            .suppliers
                            .into_iter()
                            .map(|supplier| SupplierListItemResponse {
                                supplier_id: supplier.id,
                                name: supplier.name,
                            })
                            .collect(),
                        in_series: book
                            .in_series
                            .into_iter()
                            .map(|series| SeriesListItemResponse {
                                series_id: series.series_id,
                                name: series.title,
                                column: series.column,
                            })
                            .collect(),
                        price: book.price.to_string(),
                        keywords: book
                            .keywords
                            .into_iter()
                            .map(|keyword| KeywordListItemResponse {
                                keyword_id: keyword.id,
                                keyword: keyword.keyword,
                            })
                            .collect(),
                        cover: book.cover,
                        is_onstore: book.is_onstore,
                    })
                    .collect();
                HttpResponse::Ok().json(BookTitleSearchResponse { books })
            }
            Err(e) => HttpResponse::BadGateway().body(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().body(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct BookKeywordsSearchRequest {
    keywords: String,
}

#[derive(Debug, Serialize)]
struct BookKeywordsSearchItemResponse {
    book_id: u32,
    isbn: String,
    title: String,
    authors: Vec<AuthorListItemResponse>,
    publisher: PublisherListItemResponse,
    suppliers: Vec<SupplierListItemResponse>,
    in_series: Vec<SeriesListItemResponse>,
    price: String,
    keywords: Vec<KeywordListItemResponse>,
    cover: String,
    is_onstore: bool,
}

#[derive(Debug, Serialize)]
struct BookKeywordsSearchResponse {
    books: Vec<BookKeywordsSearchItemResponse>,
}

#[post("/book/search/keywords")]
pub async fn book_keywords_search(
    pool: web::Data<Pool>,
    keywords: web::Json<BookKeywordsSearchRequest>,
) -> impl Responder {
    let request = keywords.into_inner();
    let keywords = &request.keywords;
    match pool.get_conn().await {
        Ok(mut conn) => match BookService::search_by_keywords_natural(&mut conn, keywords).await {
            Ok(books) => {
                let books = books
                    .into_iter()
                    .map(|book| BookKeywordsSearchItemResponse {
                        book_id: book.id,
                        isbn: book.isbn,
                        title: book.title,
                        authors: book
                            .authors
                            .into_iter()
                            .map(|author| AuthorListItemResponse {
                                author_id: author.id,
                                name: author.name,
                            })
                            .collect(),
                        publisher: PublisherListItemResponse {
                            publisher_id: book.publisher.id,
                            name: book.publisher.name,
                        },
                        suppliers: book
                            .suppliers
                            .into_iter()
                            .map(|supplier| SupplierListItemResponse {
                                supplier_id: supplier.id,
                                name: supplier.name,
                            })
                            .collect(),
                        in_series: book
                            .in_series
                            .into_iter()
                            .map(|series| SeriesListItemResponse {
                                series_id: series.series_id,
                                name: series.title,
                                column: series.column,
                            })
                            .collect(),
                        price: book.price.to_string(),
                        keywords: book
                            .keywords
                            .into_iter()
                            .map(|keyword| KeywordListItemResponse {
                                keyword_id: keyword.id,
                                keyword: keyword.keyword,
                            })
                            .collect(),
                        cover: book.cover,
                        is_onstore: book.is_onstore,
                    })
                    .collect();
                HttpResponse::Ok().json(BookKeywordsSearchResponse { books })
            }
            Err(e) => HttpResponse::BadGateway().body(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().body(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct BookAuthorsSearchRequest {
    authors: String,
}

#[derive(Debug, Serialize)]
struct BookAuthorsSearchItemResponse {
    book_id: u32,
    isbn: String,
    title: String,
    authors: Vec<AuthorListItemResponse>,
    publisher: PublisherListItemResponse,
    suppliers: Vec<SupplierListItemResponse>,
    in_series: Vec<SeriesListItemResponse>,
    price: String,
    keywords: Vec<KeywordListItemResponse>,
    cover: String,
    is_onstore: bool,
}

#[derive(Debug, Serialize)]
struct BookAuthorsSearchResponse {
    books: Vec<BookAuthorsSearchItemResponse>,
}

#[post("/book/search/authors")]
pub async fn book_authors_search(
    pool: web::Data<Pool>,
    authors: web::Json<BookAuthorsSearchRequest>,
) -> impl Responder {
    let request = authors.into_inner();
    let authors = &request.authors;
    match pool.get_conn().await {
        Ok(mut conn) => match BookService::search_by_authors_natural(&mut conn, authors).await {
            Ok(books) => {
                let books = books
                    .into_iter()
                    .map(|book| BookAuthorsSearchItemResponse {
                        book_id: book.id,
                        isbn: book.isbn,
                        title: book.title,
                        authors: book
                            .authors
                            .into_iter()
                            .map(|author| AuthorListItemResponse {
                                author_id: author.id,
                                name: author.name,
                            })
                            .collect(),
                        publisher: PublisherListItemResponse {
                            publisher_id: book.publisher.id,
                            name: book.publisher.name,
                        },
                        suppliers: book
                            .suppliers
                            .into_iter()
                            .map(|supplier| SupplierListItemResponse {
                                supplier_id: supplier.id,
                                name: supplier.name,
                            })
                            .collect(),
                        in_series: book
                            .in_series
                            .into_iter()
                            .map(|series| SeriesListItemResponse {
                                series_id: series.series_id,
                                name: series.title,
                                column: series.column,
                            })
                            .collect(),
                        price: book.price.to_string(),
                        keywords: book
                            .keywords
                            .into_iter()
                            .map(|keyword| KeywordListItemResponse {
                                keyword_id: keyword.id,
                                keyword: keyword.keyword,
                            })
                            .collect(),
                        cover: book.cover,
                        is_onstore: book.is_onstore,
                    })
                    .collect();
                HttpResponse::Ok().json(BookAuthorsSearchResponse { books })
            }
            Err(e) => HttpResponse::BadGateway().body(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().body(e.to_string()),
    }
}

#[derive(Debug, Serialize)]
struct AuthorListResponse {
    authors: Vec<AuthorListItemResponse>,
}

#[post("/book/author/list")]
pub async fn author_list(pool: web::Data<Pool>) -> impl Responder {
    match pool.get_conn().await {
        Ok(mut conn) => match BookService::get_author_list(&mut conn).await {
            Ok(authors) => HttpResponse::Ok().json(AuthorListResponse {
                authors: authors
                    .into_iter()
                    .map(|author| AuthorListItemResponse {
                        author_id: author.id,
                        name: author.name,
                    })
                    .collect(),
            }),
            Err(e) => HttpResponse::BadGateway().body(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().body(e.to_string()),
    }
}

#[derive(Debug, Serialize)]
struct PublisherListResponse {
    publishers: Vec<PublisherListItemResponse>,
}

#[post("/book/publisher/list")]
pub async fn publisher_list(pool: web::Data<Pool>) -> impl Responder {
    match pool.get_conn().await {
        Ok(mut conn) => match BookService::get_publisher_list(&mut conn).await {
            Ok(publishers) => HttpResponse::Ok().json(PublisherListResponse {
                publishers: publishers
                    .into_iter()
                    .map(|publisher| PublisherListItemResponse {
                        publisher_id: publisher.id,
                        name: publisher.name,
                    })
                    .collect(),
            }),
            Err(e) => HttpResponse::BadGateway().body(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().body(e.to_string()),
    }
}

#[derive(Debug, Serialize)]
struct SeriesListResponse {
    series: Vec<SeriesListItemResponse>,
}

#[post("/book/series/list")]
pub async fn series_list(pool: web::Data<Pool>) -> impl Responder {
    match pool.get_conn().await {
        Ok(mut conn) => match BookService::get_series_list(&mut conn).await {
            Ok(series) => HttpResponse::Ok().json(SeriesListResponse {
                series: series
                    .into_iter()
                    .map(|series| SeriesListItemResponse {
                        series_id: series.id,
                        name: series.title,
                        column: 0,
                    })
                    .collect(),
            }),
            Err(e) => HttpResponse::BadGateway().body(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().body(e.to_string()),
    }
}
