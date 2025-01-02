use crate::entity::{AdminRole, Location};
use crate::repo::StockRepo;
use crate::service::AdminService;
use crate::utils::Token;
use mysql_async::Conn;

pub struct StockService;

impl StockService {
    pub async fn get_location_list(
        conn: &mut Conn,
        token: &Token,
    ) -> anyhow::Result<Vec<Location>> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => StockRepo::get_location_list(conn).await,
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can get location list")
            }
        }
    }

    pub async fn change_stock(
        conn: &mut Conn,
        token: &Token,
        book_id: u32,
        location_id: u32,
        quantity: i32,
    ) -> anyhow::Result<()> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => {
                if quantity >= 0 {
                    StockRepo::in_stock(conn, book_id, location_id, quantity as u32).await
                } else {
                    StockRepo::out_stock(conn, book_id, location_id, (-quantity) as u32).await
                }
            }
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can change stock")
            }
        }
    }

    pub async fn transfer_stock(
        conn: &mut Conn,
        token: &Token,
        book_id: u32,
        from_location_id: u32,
        to_location_id: u32,
        quantity: u32,
    ) -> anyhow::Result<()> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => {
                StockRepo::out_stock(conn, book_id, from_location_id, quantity).await?;
                StockRepo::in_stock(conn, book_id, to_location_id, quantity).await
            }

            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can transfer stock")
            }
        }
    }
}
