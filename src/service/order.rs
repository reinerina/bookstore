use crate::entity::{AdminRole, Book, Order};
use crate::repo::{OrderRepo, StockRepo};
use crate::service::{AdminService, AuthService};
use crate::utils::Token;
use mysql_async::Conn;

pub struct OrderService;

impl OrderService {
    pub async fn create_order(
        conn: &mut Conn,
        token: &Token,
        books: &Vec<(u32, u32)>,
    ) -> anyhow::Result<u32> {
        let (_, username) = AuthService::verify_user(conn, token).await?;
        match OrderRepo::create_order(conn, &username, books).await? {
            Some(order) => Ok(order),
            None => anyhow::bail!("failed to create order"),
        }
    }

    pub async fn get_order_list(conn: &mut Conn, token: &Token) -> anyhow::Result<Vec<Order>> {
        let (_, username) = AuthService::verify_user(conn, token).await?;
        OrderRepo::get_order_list(conn, &username).await
    }

    pub async fn get_order_detail(
        conn: &mut Conn,
        token: &Token,
        order_id: u32,
    ) -> anyhow::Result<(Order, Vec<Book>)> {
        let (_, username) = AuthService::verify_user(conn, token).await?;
        match OrderRepo::get_order_detail(conn, &username, order_id).await? {
            (Some(order), books) => Ok((order, books)),
            (None, _) => anyhow::bail!("order {} not found", order_id),
        }
    }

    pub async fn ship_order(
        conn: &mut Conn,
        token: &Token,
        order_id: u32,
        stock_location: &Vec<(u32, u32, u32)>,
    ) -> anyhow::Result<()> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => {
                for (book_id, location_id, quantity) in stock_location.iter() {
                    match StockRepo::get_book_quantity(conn, *location_id, *book_id).await? {
                        Some(stock) => {
                            if stock < *quantity {
                                anyhow::bail!(
                                    "stock not enough for book {} in location {}",
                                    book_id,
                                    location_id
                                );
                            }
                        }
                        None => {
                            anyhow::bail!("book {} not found in location {}", book_id, location_id)
                        }
                    }
                }
                OrderRepo::ship_order(conn, order_id, stock_location).await
            }
            (_, _, false) => anyhow::bail!("permission denied: only staff or admin can ship order"),
        }
    }
}
