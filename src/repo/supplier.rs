use crate::entity::{Supplier, SupplierCatalog};
use crate::repo::BookRepo;
use mysql_async::prelude::{Query, WithParams};
use mysql_async::{params, Conn};
use mysql_common::bigdecimal::BigDecimal;

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

    pub async fn update_supplier(
        conn: &mut Conn,
        supplier_id: u32,
        name: &str,
        telephone: &str,
        email: &str,
        address: &str,
        fax: &str,
    ) -> anyhow::Result<()> {
        let query = r"UPDATE suppliers SET name=:name,telephone=:telephone,email=:email,address=:address,fax=:fax WHERE supplier_id=:supplier_id;";
        let params = params! {
            "name" => name,
            "telephone" => telephone,
            "email" => email,
            "address" => address,
            "fax" => fax,
            "supplier_id" => supplier_id,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
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

    pub async fn get_supplier_list(conn: &mut Conn) -> anyhow::Result<Vec<Supplier>> {
        let query = r"SELECT supplier_id,name,telephone,email,address,fax FROM suppliers;";
        let result = query
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

    pub async fn get_catalog_list(conn: &mut Conn) -> anyhow::Result<Vec<SupplierCatalog>> {
        let query = r"SELECT supplier_catalog_id,supplier_id,book_id,price,available_quantity FROM supplier_catalogs;";
        let result = query
            .map(
                &mut *conn,
                |(catalog_id, supplier_id, book_id, price, available_quantity)| {
                    let book_id: u32 = book_id;
                    let supplier_id: u32 = supplier_id;
                    (
                        SupplierCatalog {
                            id: catalog_id,
                            price,
                            available_quantity,
                            ..Default::default()
                        },
                        book_id,
                        supplier_id,
                    )
                },
            )
            .await?;
        let mut res = Vec::with_capacity(result.len());
        for (mut catalog, book_id, supplier_id) in result.into_iter() {
            let book = BookRepo::get_book_detail(conn, book_id).await?;
            let supplier = SupplierRepo::get_supplier(conn, supplier_id).await?;
            catalog.book = book.unwrap();
            catalog.supplier = supplier.unwrap();
            res.push(catalog);
        }
        Ok(res)
    }

    pub async fn get_supplier_catalog(
        conn: &mut Conn,
        supplier_id: u32,
        book_id: u32,
    ) -> anyhow::Result<Option<SupplierCatalog>> {
        let query = r"SELECT supplier_catalog_id,supplier_id,book_id,price,available_quantity FROM supplier_catalogs
        WHERE supplier_id=:supplier_id AND book_id=:book_id;";
        let params = params! {
            "supplier_id" => supplier_id,
            "book_id" => book_id,
        };
        let mut result = query
            .with(params)
            .map(
                &mut *conn,
                |(catalog_id, supplier_id, book_id, price, available_quantity)| {
                    let book_id: u32 = book_id;
                    let supplier_id: u32 = supplier_id;
                    (
                        SupplierCatalog {
                            id: catalog_id,
                            price,
                            available_quantity,
                            ..Default::default()
                        },
                        book_id,
                        supplier_id,
                    )
                },
            )
            .await?;
        match result.pop() {
            Some((mut catalog, book_id, supplier_id)) => {
                let book = BookRepo::get_book_detail(conn, book_id).await?;
                let supplier = SupplierRepo::get_supplier(conn, supplier_id).await?;
                catalog.book = book.unwrap();
                catalog.supplier = supplier.unwrap();
                Ok(Some(catalog))
            }
            None => Ok(None),
        }
    }

    pub async fn update_supplier_catalog(
        conn: &mut Conn,
        catalog_id: u32,
        price: BigDecimal,
        available_quantity: u32,
    ) -> anyhow::Result<()> {
        let query = r"UPDATE supplier_catalogs SET price=:price,available_quantity=:available_quantity WHERE supplier_catalog_id=:catalog_id;";
        let params = params! {
            "price" => price,
            "available_quantity" => available_quantity,
            "catalog_id" => catalog_id,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }

    pub async fn get_catalog_list_by_supplier(
        conn: &mut Conn,
        supplier_id: u32,
    ) -> anyhow::Result<Vec<SupplierCatalog>> {
        let query = r"SELECT supplier_catalog_id,supplier_id,book_id,price,available_quantity FROM supplier_catalogs
        WHERE supplier_id=:supplier_id;";
        let params = params! {
            "supplier_id" => supplier_id,
        };
        let result = query
            .with(params)
            .map(
                &mut *conn,
                |(catalog_id, supplier_id, book_id, price, available_quantity)| {
                    let book_id: u32 = book_id;
                    let supplier_id: u32 = supplier_id;
                    (
                        SupplierCatalog {
                            id: catalog_id,
                            price,
                            available_quantity,
                            ..Default::default()
                        },
                        book_id,
                        supplier_id,
                    )
                },
            )
            .await?;
        let mut res = Vec::with_capacity(result.len());
        for (mut catalog, book_id, supplier_id) in result.into_iter() {
            let book = BookRepo::get_book_detail(conn, book_id).await?;
            let supplier = SupplierRepo::get_supplier(conn, supplier_id).await?;
            catalog.book = book.unwrap();
            catalog.supplier = supplier.unwrap();
            res.push(catalog);
        }
        Ok(res)
    }

    pub async fn get_catalog_list_by_book(
        conn: &mut Conn,
        book_id: u32,
    ) -> anyhow::Result<Vec<SupplierCatalog>> {
        let query = r"SELECT supplier_catalog_id,supplier_id,book_id,price,available_quantity FROM supplier_catalogs
        WHERE book_id=:book_id;";
        let params = params! {
            "book_id" => book_id,
        };
        let result = query
            .with(params)
            .map(
                &mut *conn,
                |(catalog_id, supplier_id, book_id, price, available_quantity)| {
                    let book_id: u32 = book_id;
                    let supplier_id: u32 = supplier_id;
                    (
                        SupplierCatalog {
                            id: catalog_id,
                            price,
                            available_quantity,
                            ..Default::default()
                        },
                        book_id,
                        supplier_id,
                    )
                },
            )
            .await?;
        let mut res = Vec::with_capacity(result.len());
        for (mut catalog, book_id, supplier_id) in result.into_iter() {
            let book = BookRepo::get_book_detail(conn, book_id).await?;
            let supplier = SupplierRepo::get_supplier(conn, supplier_id).await?;
            catalog.book = book.unwrap();
            catalog.supplier = supplier.unwrap();
            res.push(catalog);
        }
        Ok(res)
    }

    pub async fn delete_supplier_catalog(conn: &mut Conn, catalog_id: u32) -> anyhow::Result<()> {
        let query = r"DELETE FROM supplier_catalogs WHERE supplier_catalog_id=:catalog_id;";
        let params = params! {
            "catalog_id" => catalog_id,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }

    pub async fn delete_supplier(conn: &mut Conn, supplier_id: u32) -> anyhow::Result<()> {
        let query = r"DELETE FROM suppliers WHERE supplier_id=:supplier_id;";
        let params = params! {
            "supplier_id" => supplier_id,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }

    pub async fn add_supplier_catalog(
        conn: &mut Conn,
        supplier_id: u32,
        book_id: u32,
        price: BigDecimal,
        available_quantity: u32,
    ) -> anyhow::Result<Option<u32>> {
        let query = r"INSERT INTO supplier_catalogs(supplier_id,book_id,price,available_quantity) VALUES(:supplier_id,:book_id,:price,:available_quantity);";
        let params = params! {
            "supplier_id" => supplier_id,
            "book_id" => book_id,
            "price" => price,
            "available_quantity" => available_quantity,
        };
        query.with(params).run(&mut *conn).await?;
        let query = r"SELECT LAST_INSERT_ID() as catalog_id;";
        let catalog_id = query.with(()).first::<u32, &mut Conn>(conn).await?;
        Ok(catalog_id)
    }
}
