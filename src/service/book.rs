use crate::entity::{AdminRole, Author, Book, Keyword, Publisher, Series};
use crate::repo::BookRepo;
use crate::service::AdminService;
use crate::utils::Token;
use mysql_async::Conn;
use mysql_common::bigdecimal::BigDecimal;

pub struct BookService;

impl BookService {
    pub async fn get_book_detail(conn: &mut Conn, book_id: u32) -> anyhow::Result<Book> {
        match BookRepo::get_book_detail(conn, book_id).await? {
            None => anyhow::bail!("book {} not found", book_id),
            Some(book) => Ok(book),
        }
    }

    pub async fn get_book_list(conn: &mut Conn) -> anyhow::Result<Vec<Book>> {
        BookRepo::get_book_list(conn).await
    }

    pub async fn get_keyword_list(conn: &mut Conn) -> anyhow::Result<Vec<Keyword>> {
        BookRepo::get_keyword_list(conn).await
    }

    pub async fn get_author_list(conn: &mut Conn) -> anyhow::Result<Vec<Author>> {
        BookRepo::get_author_list(conn).await
    }

    pub async fn get_publisher_list(conn: &mut Conn) -> anyhow::Result<Vec<Publisher>> {
        BookRepo::get_publisher_list(conn).await
    }

    pub async fn get_series_list(conn: &mut Conn) -> anyhow::Result<Vec<Series>> {
        BookRepo::get_series_list(conn).await
    }

    pub async fn add_keyword(conn: &mut Conn, token: &Token, keyword: &str) -> anyhow::Result<u32> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => match BookRepo::add_keyword(conn, keyword).await? {
                Some(keyword_id) => Ok(keyword_id),
                None => anyhow::bail!("add keyword failed"),
            },
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can add keyword")
            }
        }
    }

    pub async fn add_author(conn: &mut Conn, token: &Token, author: &str) -> anyhow::Result<u32> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => match BookRepo::add_author(conn, author).await? {
                Some(author_id) => Ok(author_id),
                None => anyhow::bail!("add author failed"),
            },
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can add author")
            }
        }
    }

    pub async fn add_publisher(
        conn: &mut Conn,
        token: &Token,
        publisher: &str,
    ) -> anyhow::Result<u32> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => match BookRepo::add_publisher(conn, publisher).await? {
                Some(publisher_id) => Ok(publisher_id),
                None => anyhow::bail!("add publisher failed"),
            },
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can add publisher")
            }
        }
    }

    pub async fn add_book(
        conn: &mut Conn,
        token: &Token,
        isbn: &str,
        title: &str,
        authors: &Vec<u32>,
        keywords: &Vec<u32>,
        series: &Vec<(u32, u32)>,
        suppliers: &Vec<u32>,
        publisher: u32,
        price: BigDecimal,
        catalog: &str,
        cover: &str,
        is_onstore: bool,
    ) -> anyhow::Result<u32> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => {
                match BookRepo::add_book(
                    conn, isbn, title, authors, keywords, series, suppliers, publisher, price,
                    catalog, cover, is_onstore,
                )
                .await?
                {
                    Some(book_id) => Ok(book_id),
                    None => anyhow::bail!("add book failed"),
                }
            }
            (_, _, false) => anyhow::bail!("permission denied: only staff or admin can add book"),
        }
    }

    pub async fn update_book(
        conn: &mut Conn,
        token: &Token,
        book_id: u32,
        isbn: &str,
        title: &str,
        authors: &Vec<u32>,
        keywords: &Vec<u32>,
        series: &Vec<(u32, u32)>,
        suppliers: &Vec<u32>,
        publisher: u32,
        price: BigDecimal,
        catalog: &str,
        cover: &str,
        is_onstore: bool,
    ) -> anyhow::Result<()> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => {
                match BookRepo::update_book(
                    conn, book_id, isbn, title, authors, keywords, series, suppliers, publisher,
                    price, catalog, cover, is_onstore,
                )
                .await?
                {
                    Some(_) => Ok(()),
                    None => anyhow::bail!("update book failed"),
                }
            }
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can update book")
            }
        }
    }

    pub async fn search_by_title_natural(
        conn: &mut Conn,
        title: &str,
    ) -> anyhow::Result<Vec<Book>> {
        BookRepo::search_by_title_natural(conn, title).await
    }

    pub async fn search_by_keywords_natural(
        conn: &mut Conn,
        keywords: &str,
    ) -> anyhow::Result<Vec<Book>> {
        let keywords = keywords
            .split(|c| {
                let c: char = c;
                c == ',' || c == ';' || c.is_whitespace()
            })
            .map(|s| format!("{} ", s))
            .collect::<String>();
        let keywords = keywords.trim();
        BookRepo::search_by_keyword_natural(conn, keywords).await
    }

    pub async fn search_by_authors_natural(
        conn: &mut Conn,
        authors: &str,
    ) -> anyhow::Result<Vec<Book>> {
        let authors = authors
            .split(|c| {
                let c: char = c;
                c == ',' || c == ';' || c.is_whitespace()
            })
            .map(|s| format!("{} ", s))
            .collect::<String>();
        let authors = authors.trim();
        BookRepo::search_by_author_natural(conn, authors).await
    }
}
