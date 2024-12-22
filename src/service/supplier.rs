use crate::entity::{AdminRole, Supplier, SupplierCatalog};
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

    pub async fn get_supplier_list(conn: &mut Conn) -> anyhow::Result<Vec<Supplier>> {
        SupplierRepo::get_supplier_list(conn).await
    }

    pub async fn update_supplier(
        conn: &mut Conn,
        supplier_id: u32,
        name: &str,
        telephone: &str,
        email: &str,
        address: &str,
        fax: &str,
        token: &Token,
    ) -> anyhow::Result<()> {
        match AdminService::verify_admin(conn, token, AdminRole::Admin).await? {
            (_, _, true) => {
                SupplierRepo::update_supplier(
                    conn,
                    supplier_id,
                    name,
                    telephone,
                    email,
                    address,
                    fax,
                )
                .await
            }
            (_, _, false) => anyhow::bail!("permission denied: only admin can update supplier"),
        }
    }

    pub async fn delete_supplier(
        conn: &mut Conn,
        supplier_id: u32,
        token: &Token,
    ) -> anyhow::Result<()> {
        match AdminService::verify_admin(conn, token, AdminRole::Admin).await? {
            (_, _, true) => SupplierRepo::delete_supplier(conn, supplier_id).await,
            (_, _, false) => anyhow::bail!("permission denied: only admin can delete supplier"),
        }
    }

    pub async fn get_supplier_catalog_list(
        conn: &mut Conn,
        token: &Token,
    ) -> anyhow::Result<Vec<SupplierCatalog>> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => SupplierRepo::get_catalog_list(conn).await,
            (_, _, false) => {
                anyhow::bail!(
                    "permission denied: only staff or admin can get supplier catalog list"
                )
            }
        }
    }

    pub async fn get_supplier_catalog_list_by_supplier(
        conn: &mut Conn,
        supplier_id: u32,
        token: &Token,
    ) -> anyhow::Result<Vec<SupplierCatalog>> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => SupplierRepo::get_catalog_list_by_supplier(conn, supplier_id).await,
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can get supplier catalog")
            }
        }
    }

    pub async fn get_supplier_catalog_list_by_book(
        conn: &mut Conn,
        book_id: u32,
        token: &Token,
    ) -> anyhow::Result<Vec<SupplierCatalog>> {
        match AdminService::verify_admin(conn, token, AdminRole::Staff).await? {
            (_, _, true) => SupplierRepo::get_catalog_list_by_book(conn, book_id).await,
            (_, _, false) => {
                anyhow::bail!("permission denied: only staff or admin can get supplier catalog")
            }
        }
    }
}
