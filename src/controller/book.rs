use crate::service::BookService;
use crate::utils::Token;
use actix_web::{post, web, HttpResponse, Responder};
use mysql_async::Pool;
use serde::{Deserialize, Serialize};
use std::mem::take;

#[derive(Debug, Serialize)]
struct AuthorDetailResponse {
    author_id: u32,
    name: String,
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
    keywords: Vec<String>,
    catalog: String,
    cover: String,
    is_onstore: bool,
}

#[post("/book/{id}/detail")]
pub async fn book_detail(pool: web::Data<Pool>, id: web::Path<(u32,)>) -> impl Responder {
    let mut conn = pool.get_conn().await.unwrap();
    match BookService::get_book_detail(&mut conn, id.into_inner().0).await {
        Ok(mut book) => HttpResponse::Ok().json(BookDetailResponse {
            book_id: book.id,
            isbn: book.isbn,
            title: book.title,
            authors: book
                .authors
                .iter_mut()
                .map(|author| AuthorDetailResponse {
                    author_id: author.id,
                    name: take(&mut author.name),
                })
                .collect(),
            publisher: PublisherDetailResponse {
                publisher_id: book.publisher.id,
                name: take(&mut book.publisher.name),
            },
            suppliers: book
                .suppliers
                .iter_mut()
                .map(|supplier| SupplierDetailResponse {
                    supplier_id: supplier.id,
                    name: take(&mut supplier.name),
                    telephone: take(&mut supplier.telephone),
                    email: take(&mut supplier.email),
                    address: take(&mut supplier.address),
                    fax: take(&mut supplier.fax),
                })
                .collect(),
            in_series: book
                .in_series
                .iter_mut()
                .map(|series| SeriesDetailResponse {
                    series_id: series.series_id,
                    name: take(&mut series.title),
                    column: series.column,
                })
                .collect(),
            price: book.price.to_string(),
            keywords: book.keywords.clone(),
            catalog: book.catalog,
            cover: book.cover,
            is_onstore: book.is_onstore,
        }),
        Err(e) => HttpResponse::BadGateway().body(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct BookShortageCreateRequest {
    book_suppliers: Vec<(u32, u32, u32)>,
    token: String,
    tag: String,
    nonce: String,
}

#[derive(Debug, Serialize)]
struct BookShortageCreateResponse {
    shortage_id: u32,
}

#[post("/book/shortage/create")]
pub async fn book_shortage_create(
    pool: web::Data<Pool>,
    mut book_shortage_create_request: web::Json<BookShortageCreateRequest>,
) -> impl Responder {
    let token = &Token {
        token: take(&mut book_shortage_create_request.token),
        tag: take(&mut book_shortage_create_request.tag),
        nonce: take(&mut book_shortage_create_request.nonce),
    };
    let book_suppliers = &book_shortage_create_request.book_suppliers;
    let mut conn = pool.get_conn().await.unwrap();

    match BookService::create_book_shortage(&mut conn, token, book_suppliers).await {
        Ok(shortage_id) => HttpResponse::Ok().json(BookShortageCreateResponse { shortage_id }),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
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
    keywords: Vec<String>,
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
            Ok(mut books) => {
                let books = books
                    .iter_mut()
                    .map(|book| BookListItemResponse {
                        book_id: book.id,
                        isbn: take(&mut book.isbn),
                        title: take(&mut book.title),
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
                            name: take(&mut book.publisher.name),
                        },
                        suppliers: book
                            .suppliers
                            .iter_mut()
                            .map(|supplier| SupplierListItemResponse {
                                supplier_id: supplier.id,
                                name: take(&mut supplier.name),
                            })
                            .collect(),
                        in_series: book
                            .in_series
                            .iter_mut()
                            .map(|series| SeriesListItemResponse {
                                series_id: series.series_id,
                                name: take(&mut series.title),
                                column: series.column,
                            })
                            .collect(),
                        price: book.price.to_string(),
                        keywords: take(&mut book.keywords),
                        cover: take(&mut book.cover),
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
            Ok(mut keywords) => HttpResponse::Ok().json(KeywordListResponse {
                keywords: keywords
                    .iter_mut()
                    .map(|keyword| KeywordListItemResponse {
                        keyword_id: keyword.id,
                        keyword: take(&mut keyword.keyword),
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
    mut keyword_add_request: web::Json<KeywordAddRequest>,
) -> impl Responder {
    let token = &Token {
        token: take(&mut keyword_add_request.token),
        tag: take(&mut keyword_add_request.tag),
        nonce: take(&mut keyword_add_request.nonce),
    };
    let keyword = &keyword_add_request.keyword;
    match pool.get_conn().await {
        Ok(mut conn) => match BookService::add_keyword(&mut conn, token, keyword).await {
            Ok(keyword_id) => HttpResponse::Ok().json(KeywordAddResponse { keyword_id }),
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().body(e.to_string()),
    }
}
