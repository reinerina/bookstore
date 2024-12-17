use crate::entity::Supplier;
use mysql_async::prelude::{Query, WithParams};
use mysql_async::{params, Conn};

pub struct SupplierRepo;

impl SupplierRepo {
    pub async fn get_supplier(
        conn: &mut Conn,
        supplier_id: u32,
    ) -> anyhow::Result<Option<Supplier>> {
        let query = r"SELECT supplier_id,name,telephone,email,address,fax FROM suppliers
        WHERE supplier_id=:supplier_id;";
        let params = params! {
            "supplier_id" => supplier_id,
        };
        let mut result = query
            .with(params)
            .map(
                conn,
                |(supplier_id, name, telephone, email, address, fax)| Supplier {
                    id: supplier_id,
                    name,
                    telephone,
                    email,
                    address,
                    fax,
                },
            )
            .await?;

        Ok(result.pop())
    }

    pub async fn add_supplier(
        conn: &mut Conn,
        name: &str,
        telephone: &str,
        email: &str,
        address: &str,
        fax: &str,
    ) -> anyhow::Result<Option<u32>> {
        let query = r"INSERT INTO suppliers(name,telephone,email,address,fax) VALUES(:name,:telephone,:email,:address,:fax);";
        let params = params! {
            "name" => name,
            "telephone" => telephone,
            "email" => email,
            "address" => address,
            "fax" => fax,
        };
        query.with(params).run(&mut *conn).await?;
        let query = r"SELECT LAST_INSERT_ID() as supplier_id;";
        let supplier_id = query.with(()).first::<u32, &mut Conn>(conn).await?;
        Ok(supplier_id)
    }

    pub async fn get_available_suppliers(
        conn: &mut Conn,
        book_id: u32,
        quantity: u32,
    ) -> anyhow::Result<Vec<Supplier>> {
        let query = r"SELECT
	suppliers.supplier_id,
	suppliers.`name`,
	suppliers.telephone,
	suppliers.email,
	suppliers.address,
	suppliers.fax
FROM
	suppliers
	LEFT JOIN supplier_catalogs ON suppliers.supplier_id = supplier_catalogs.supplier_id
WHERE
	supplier_catalogs.book_id = :book_id
	AND supplier_catalogs.available_quantity >= :quantity;";
        let params = params! {
            "book_id" => book_id,
            "quantity" => quantity,
        };
        let result = query
            .with(params)
            .map(
                conn,
                |(supplier_id, name, telephone, email, address, fax)| Supplier {
                    id: supplier_id,
                    name,
                    telephone,
                    email,
                    address,
                    fax,
                },
            )
            .await?;
        Ok(result)
    }
}
