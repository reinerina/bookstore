use crate::entity::{AdminRole, Book, Keyword};
use crate::repo::BookRepo;
use crate::service::AdminService;
use crate::utils::Token;
use mysql_async::Conn;

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
