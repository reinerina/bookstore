use crate::entity::{AdminRole, Shortage};
use crate::repo::{ShortageRepo, SupplierRepo};
use crate::service::AdminService;
use crate::utils::Token;
use mysql_async::Conn;

pub struct ShortageService;

impl ShortageService {
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

                match ShortageRepo::create_book_shortage(conn, book_suppliers).await? {
                    Some(shortage_id) => Ok(shortage_id),
                    None => anyhow::bail!("create shortage failed"),
                }
            }
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can create shortage")
            }
        }
    }

    pub async fn get_shortage_list(
        conn: &mut Conn,
        token: &Token,
    ) -> anyhow::Result<Vec<Shortage>> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => ShortageRepo::get_shortage_list(conn).await,
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can view shortage list")
            }
        }
    }

    pub async fn get_shortage_detail(
        conn: &mut Conn,
        shortage_id: u32,
        token: &Token,
    ) -> anyhow::Result<Shortage> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => match ShortageRepo::get_shortage_detail(conn, shortage_id).await? {
                Some(shortage) => Ok(shortage),
                None => anyhow::bail!("shortage {} not found", shortage_id),
            },
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can view shortage")
            }
        }
    }
}
