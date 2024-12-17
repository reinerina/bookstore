use crate::entity::{AdminRole, Book, Keyword};
use crate::repo::{BookRepo, SupplierRepo};
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

    pub async fn create_book_shortage(
        conn: &mut Conn,
        token: &Token,
        book_suppliers: &Vec<(u32, u32, u32)>,
    ) -> anyhow::Result<u32> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => {
                for (book_id, supplier_id, quantity) in book_suppliers.iter() {
                    let suppliers =
                        SupplierRepo::get_available_suppliers(conn, *book_id, *quantity).await?;

                    if !suppliers.iter().any(|s| s.id == *supplier_id) {
                        anyhow::bail!(
                            "supplier {} is not available for book {}",
                            supplier_id,
                            book_id
                        );
                    }
                }

                match BookRepo::create_book_shortage(conn, book_suppliers).await? {
                    Some(shortage_id) => Ok(shortage_id),
                    None => anyhow::bail!("create shortage failed"),
                }
            }
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can create shortage")
            }
        }
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
            (_, _, false) => anyhow::bail!("permission denied: only staff or admin can add keyword"),
        }
    }
}
