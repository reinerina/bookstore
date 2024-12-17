use crate::entity::{AdminRole, Supplier};
use crate::repo::SupplierRepo;
use crate::service::AdminService;
use crate::utils::Token;
use mysql_async::Conn;

pub struct SupplierService;

impl SupplierService {
    pub async fn get_supplier(conn: &mut Conn, supplier_id: u32) -> anyhow::Result<Supplier> {
        match SupplierRepo::get_supplier(conn, supplier_id).await {
            Ok(supplier) => match supplier {
                None => anyhow::bail!("supplier {} not found", supplier_id),
                Some(supplier) => Ok(supplier),
            },
            Err(e) => anyhow::bail!(e),
        }
    }

    pub async fn add_supplier(
        conn: &mut Conn,
        name: &str,
        telephone: &str,
        email: &str,
        address: &str,
        fax: &str,
        token: &Token,
    ) -> anyhow::Result<u32> {
        match AdminService::verify_admin(conn, token, AdminRole::Admin).await? {
            (_, _, true) => {
                match SupplierRepo::add_supplier(conn, name, telephone, email, address, fax).await?
                {
                    None => anyhow::bail!("add supplier failed"),
                    Some(supplier_id) => Ok(supplier_id),
                }
            }
            (_, _, false) => anyhow::bail!("permission denied: only admin can add supplier"),
        }
    }
}
