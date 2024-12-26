use crate::entity::{Admin, AdminRole, Book, Customer, Location, Order};
use crate::repo::{AdminRepo, BookRepo, OrderRepo, StockRepo, UserRepo, UtilsRepo};
use crate::utils::{encrypt_admin_password, generate_token, validate_token, Token};
use mysql_async::Conn;
use mysql_common::bigdecimal::BigDecimal;

pub struct AdminService;

impl AdminService {
    pub async fn register(
        conn: &mut Conn,
        username: &str,
        password: &str,
        role: AdminRole,
        token: &Token,
    ) -> anyhow::Result<u32> {
        let password = encrypt_admin_password(password).await?;

        match AdminService::verify_admin(conn, token, AdminRole::Admin).await? {
            (_, _, true) => match AdminRepo::register_admin(conn, username, &password, role).await?
            {
                Some(admin_id) => Ok(admin_id),
                None => anyhow::bail!("register failed"),
            },
            (_, _, false) => anyhow::bail!("permission denied: only admin can register admin user"),
        }
    }

    pub async fn login(conn: &mut Conn, username: &str, password: &str) -> anyhow::Result<Token> {
        let password = encrypt_admin_password(password).await?;

        match AdminRepo::login_admin(conn, username, &password).await? {
            true => {
                let token = generate_token(username).await?;
                Ok(token)
            }
            false => anyhow::bail!("login failed, username or password incorrect"),
        }
    }

    pub async fn get_admin_detail(conn: &mut Conn, token: &Token) -> anyhow::Result<Admin> {
        let username = validate_token(token).await?;
        match AdminRepo::get_admin_detail(conn, &username).await? {
            Some(admin) => Ok(admin),
            None => anyhow::bail!("admin {} not found", username),
        }
    }

    pub async fn get_book_detail(
        conn: &mut Conn,
        token: &Token,
        book_id: u32,
    ) -> anyhow::Result<(Book, Vec<Location>)> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => match BookRepo::get_book_detail(conn, book_id).await? {
                Some(book) => {
                    let locations = StockRepo::get_location_list_by_book(conn, book_id).await?;
                    Ok((book, locations))
                }
                None => anyhow::bail!("book {} not found", book_id),
            },
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can get book detail")
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
                UtilsRepo::transaction(conn).await?;
                match BookRepo::add_book(
                    conn, isbn, title, authors, keywords, series, suppliers, publisher, price,
                    catalog, cover, is_onstore,
                )
                .await?
                {
                    Some(book_id) => {
                        UtilsRepo::commit(conn).await?;
                        Ok(book_id)
                    }
                    None => {
                        UtilsRepo::rollback(conn).await?;
                        anyhow::bail!("add book failed")
                    }
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
                UtilsRepo::transaction(conn).await?;
                match BookRepo::update_book(
                    conn, book_id, isbn, title, authors, keywords, series, suppliers, publisher,
                    price, catalog, cover, is_onstore,
                )
                .await
                {
                    Ok(_) => {
                        UtilsRepo::commit(conn).await?;
                    }
                    Err(e) => {
                        UtilsRepo::rollback(conn).await?;
                        anyhow::bail!(e);
                    }
                }
                Ok(())
            }
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can update book")
            }
        }
    }

    pub async fn get_customer_list(
        conn: &mut Conn,
        token: &Token,
    ) -> anyhow::Result<Vec<Customer>> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => UserRepo::get_user_list(conn).await,
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can get customer list")
            }
        }
    }

    pub async fn set_customer_balance(
        conn: &mut Conn,
        token: &Token,
        customer_id: u32,
        balance: BigDecimal,
    ) -> anyhow::Result<()> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => UserRepo::set_user_balance(conn, customer_id, balance).await,
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can set customer balance")
            }
        }
    }

    pub async fn set_customer_credit_level(
        conn: &mut Conn,
        token: &Token,
        customer_id: u32,
        credit_level: u32,
    ) -> anyhow::Result<()> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => UserRepo::set_user_credit_level(conn, customer_id, credit_level).await,
            (_, _, false) => {
                anyhow::bail!(
                    "permission denied: only staff or admin can set customer credit level"
                )
            }
        }
    }

    pub async fn get_customer_order_list(
        conn: &mut Conn,
        token: &Token,
    ) -> anyhow::Result<Vec<Order>> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => OrderRepo::get_order_list_all(conn).await,
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can get customer order list")
            }
        }
    }

    pub async fn ship_order_auto(
        conn: &mut Conn,
        token: &Token,
        order_id: u32,
    ) -> anyhow::Result<()> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => {
                UtilsRepo::transaction(conn).await?;
                match OrderRepo::ship_order_automatic(conn, order_id).await {
                    Ok(_) => {
                        UtilsRepo::commit(conn).await?;
                    }
                    Err(e) => {
                        UtilsRepo::rollback(conn).await?;
                        anyhow::bail!(e);
                    }
                }
                Ok(())
            }
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can ship order")
            }
        }
    }

    pub async fn verify_admin(
        conn: &mut Conn,
        token: &Token,
        role: AdminRole,
    ) -> anyhow::Result<(u32, String, bool)> {
        let username = validate_token(token).await?;
        match AdminRepo::get_admin_detail(conn, &username).await? {
            Some(admin) => Ok((admin.id, admin.username, admin.role >= role)),
            None => anyhow::bail!("admin {} not found", username),
        }
    }

    pub async fn get_book_list(conn: &mut Conn) -> anyhow::Result<Vec<Book>> {
        BookRepo::get_book_list(conn).await
    }
}
