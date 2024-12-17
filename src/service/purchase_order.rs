use crate::entity::{
    AdminRole, Book, PurchaseOrder, PurchaseOrderStatus, Supplier, SupplierCatalog,
};
use crate::repo::{PurchaseOrderRepo, StockRepo};
use crate::service::AdminService;
use crate::utils::Token;
use mysql_async::Conn;

pub struct PurchaseOrderService;

impl PurchaseOrderService {
    pub async fn confirm_purchase_order(
        conn: &mut Conn,
        token: &Token,
        purchase_order_id: u32,
        stocks: &Vec<(u32, Vec<(u32, u32)>)>,
    ) -> anyhow::Result<()> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => {
                let (purchase_order, _, _, books) =
                    match PurchaseOrderRepo::get_purchase_order_detail(conn, purchase_order_id)
                        .await?
                    {
                        Some(purchase_order) => purchase_order,
                        None => anyhow::bail!("purchase order {} not found", purchase_order_id),
                    };

                if purchase_order.status.ne(&PurchaseOrderStatus::Received) {
                    anyhow::bail!(
                        "purchase order {} is not received, current status: {}",
                        purchase_order_id,
                        purchase_order.status.to_string()
                    );
                }

                let total_stock_quantity = stocks
                    .iter()
                    .map(|(_, stocks)| stocks.iter().map(|(_, quantity)| quantity).sum::<u32>())
                    .sum::<u32>();

                let total_purchase_quantity = purchase_order
                    .items
                    .iter()
                    .map(|item| item.quantity)
                    .sum::<u32>();

                if total_stock_quantity.ne(&total_purchase_quantity) {
                    anyhow::bail!(
                        "total stock quantity {} is not equal to total purchase quantity {}",
                        total_stock_quantity,
                        total_purchase_quantity
                    );
                }

                for (book_id, stocks) in stocks {
                    match books.iter().find(|book| book.id.eq(book_id)) {
                        Some(_) => {
                            for (location_id, quantity) in stocks {
                                StockRepo::in_stock(conn, *location_id, *book_id, *quantity)
                                    .await?;
                            }
                        }
                        None => anyhow::bail!("book {} not found in purchase order", book_id),
                    };
                }
            }
            (_, _, false) => {
                anyhow::bail!("permission denied: only admin can confirm purchase order")
            }
        }

        Ok(())
    }

    pub async fn get_purchase_order_detail(
        conn: &mut Conn,
        token: &Token,
        purchase_order_id: u32,
    ) -> anyhow::Result<(
        PurchaseOrder,
        Vec<SupplierCatalog>,
        Vec<Supplier>,
        Vec<Book>,
    )> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => {
                match PurchaseOrderRepo::get_purchase_order_detail(conn, purchase_order_id).await? {
                    Some((purchase_order, catalogs, suppliers, books)) => {
                        Ok((purchase_order, catalogs, suppliers, books))
                    }
                    None => anyhow::bail!("purchase order {} not found", purchase_order_id),
                }
            }
            (_, _, false) => {
                anyhow::bail!("permission denied: only admin can get purchase order detail")
            }
        }
    }

    pub async fn get_purchase_order_list(
        conn: &mut Conn,
        token: &Token,
    ) -> anyhow::Result<Vec<PurchaseOrder>> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => PurchaseOrderRepo::get_purchase_order_list(conn).await,
            (_, _, false) => {
                anyhow::bail!("permission denied: only admin can get purchase order list")
            }
        }
    }

    pub async fn create_purchase_order(
        conn: &mut Conn,
        token: &Token,
        shortage_id: u32,
    ) -> anyhow::Result<u32> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => {
                match PurchaseOrderRepo::create_purchase_order(conn, shortage_id).await? {
                    Some(purchase_order_id) => Ok(purchase_order_id),
                    None => anyhow::bail!("create purchase order failed"),
                }
            }
            (_, _, false) => {
                anyhow::bail!("permission denied: only admin can create purchase order")
            }
        }
    }
}
