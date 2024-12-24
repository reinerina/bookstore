use crate::entity::{
    Book, Publisher, PurchaseOrder, PurchaseOrderItem, ShortageItem, Supplier, SupplierCatalog,
};
use mysql_async::prelude::{Query, WithParams};
use mysql_async::{params, Conn};

pub struct PurchaseOrderRepo;

impl PurchaseOrderRepo {
    pub async fn complete_purchase_order(
        conn: &mut Conn,
        stock: &Vec<(u32, u32, u32)>,
        order_id: u32,
    ) -> anyhow::Result<()> {
        for (book_id, location_id, quantity) in stock.iter() {
            let query = r"INSERT INTO book_locations (book_id, location_id, quantity) VALUES (:book_id, :location_id, :quantity)
            ON DUPLICATE KEY UPDATE quantity = quantity + :quantity";
            let params = params! {
                "book_id" => book_id,
                "location_id" => location_id,
                "quantity" => quantity,
            };
            query.with(params).run(&mut *conn).await?;
        }
        let query = "UPDATE purchase_orders SET status = 'completed' WHERE id = :id";
        let params = params! {
            "id" => order_id,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }

    pub async fn create_purchase_order(
        conn: &mut Conn,
        shortage_id: u32,
    ) -> anyhow::Result<Option<u32>> {
        let query = r"INSERT INTO purchase_orders (order_date, expected_delivery_date) VALUES (NOW(), NOW() + INTERVAL 7 DAY)";
        match query.with(()).first::<u32, &mut Conn>(conn).await? {
            Some(purchase_order_id) => {
                let query = r"SELECT shortage_item_id, shortage_id, book_id, supplier_id, shortage FROM book_shortages
                WHERE shortage_id = :shortage_id";
                let params = params! {
                    "shortage_id" => shortage_id,
                };
                let shortage_items = query
                    .with(params)
                    .map(
                        &mut *conn,
                        |(shortage_item_id, shortage_id, book_id, supplier_id, shortage)| {
                            ShortageItem {
                                id: shortage_item_id,
                                shortage_id,
                                book_id,
                                supplier_id,
                                shortage,
                            }
                        },
                    )
                    .await?;
                for item in shortage_items {
                    let query = r"SELECT supplier_catalog_id FROM supplier_catalogs
                    WHERE supplier_id = :supplier_id AND book_id = :book_id";
                    let params = params! {
                        "supplier_id" => item.supplier_id,
                        "book_id" => item.book_id,
                    };
                    match query.with(params).first::<u32, &mut Conn>(conn).await? {
                        Some(supplier_catalog_id) => {
                            let query = r"UPDATE supplier_catalogs SET available_quantity = available_quantity - :quantity
                            WHERE supplier_catalog_id = :supplier_catalog_id";
                            let params = params! {
                                "supplier_catalog_id" => supplier_catalog_id,
                                "quantity" => item.shortage,
                            };

                            query.with(params).run(&mut *conn).await?;
                            let query = r"INSERT INTO purchase_order_items (purchase_order_id, supplier_catalog_id, quantity) VALUES (:purchase_order_id, :supplier_catalog_id, :quantity)";
                            let params = params! {
                                "purchase_order_id" => purchase_order_id,
                                "supplier_catalog_id" => supplier_catalog_id,
                                "quantity" => item.shortage,
                            };
                            query.with(params).run(&mut *conn).await?;
                        }
                        None => anyhow::bail!("failed to find supplier catalog"),
                    }
                }
                Ok(Some(purchase_order_id))
            }
            None => anyhow::bail!("failed to create purchase order"),
        }
    }

    pub async fn get_purchase_order_list(conn: &mut Conn) -> anyhow::Result<Vec<PurchaseOrder>> {
        let query = r"SELECT
	purchase_orders.purchase_order_id,
	purchase_orders.order_date,
	purchase_orders.expected_delivery_date,
	purchase_orders.`status`,
	SUM( supplier_catalogs.price * purchase_order_items.quantity ) AS total_amount
FROM
	purchase_orders
	LEFT JOIN purchase_order_items ON purchase_orders.purchase_order_id = purchase_order_items.purchase_order_id
	LEFT JOIN supplier_catalogs ON purchase_order_items.supplier_catalog_id = purchase_order_items.supplier_catalog_id
GROUP BY
	purchase_orders.purchase_order_id;";
        let purchase_orders = query
            .with(())
            .map(
                conn,
                |(purchase_order_id, order_date, expected_delivery_date, status, total_amount)| {
                    PurchaseOrder {
                        id: purchase_order_id,
                        order_date,
                        expected_delivery_date,
                        status: {
                            let status: String = status;
                            status.parse().unwrap()
                        },
                        items: Vec::new(),
                        total_amount,
                    }
                },
            )
            .await?;
        Ok(purchase_orders)
    }

    pub async fn get_purchase_order_detail(
        conn: &mut Conn,
        purchase_order_id: u32,
    ) -> anyhow::Result<Option<(PurchaseOrder, Vec<SupplierCatalog>)>> {
        let query = r"SELECT
	purchase_orders.purchase_order_id,
	purchase_orders.order_date,
	purchase_orders.expected_delivery_date,
	purchase_orders.`status`,
	SUM( supplier_catalogs.price * purchase_order_items.quantity ) AS total_amount
FROM
	purchase_orders
	LEFT JOIN purchase_order_items ON purchase_orders.purchase_order_id = purchase_order_items.purchase_order_id
	LEFT JOIN supplier_catalogs ON purchase_order_items.supplier_catalog_id = purchase_order_items.supplier_catalog_id
WHERE
    purchase_orders.purchase_order_id = :purchase_order_id
GROUP BY
	purchase_orders.purchase_order_id;";
        let params = params! {
            "purchase_order_id" => purchase_order_id,
        };
        let mut purchase_order = query
            .with(params)
            .map(
                &mut *conn,
                |(purchase_order_id, order_date, expected_delivery_date, status, total_amount)| {
                    PurchaseOrder {
                        id: purchase_order_id,
                        order_date,
                        expected_delivery_date,
                        status: {
                            let status: String = status;
                            status.parse().unwrap()
                        },
                        items: Vec::new(),
                        total_amount,
                    }
                },
            )
            .await?;
        let query = r"SELECT
	purchase_order_items.order_item_id,
	purchase_order_items.supplier_catalog_id,
	purchase_order_items.quantity,
	supplier_catalogs.price,
	supplier_catalogs.price * purchase_order_items.quantity,
	books.book_id,
	books.title,
	books.isbn,
	suppliers.supplier_id,
	suppliers.`name`,
	publishers.publisher_id,
	publishers.`name`
FROM
	purchase_order_items
	LEFT JOIN supplier_catalogs ON purchase_order_items.supplier_catalog_id = supplier_catalogs.supplier_catalog_id
	LEFT JOIN books ON supplier_catalogs.book_id = books.book_id
	LEFT JOIN suppliers ON supplier_catalogs.supplier_id = suppliers.supplier_id
	LEFT JOIN publishers ON books.publisher_id = publishers.publisher_id
WHERE
	purchase_order_items.purchase_order_id = :purchase_order_id;";
        let params = params! {
            "purchase_order_id" => purchase_order_id,
        };
        let items = query
            .with(params)
            .map(
                conn,
                |(
                    order_item_id,
                    supplier_catalog_id,
                    quantity,
                    price,
                    total_price,
                    book_id,
                    title,
                    isbn,
                    supplier_id,
                    supplier_name,
                    publisher_id,
                    publisher_name,
                )| {
                    (
                        PurchaseOrderItem {
                            id: order_item_id,
                            purchase_order_id,
                            supplier_catalog_id,
                            quantity,
                            total_price,
                        },
                        SupplierCatalog {
                            id: supplier_catalog_id,
                            supplier: Supplier {
                                id: supplier_id,
                                name: supplier_name,
                                ..Default::default()
                            },
                            book: Book {
                                id: book_id,
                                title,
                                isbn,
                                publisher: Publisher {
                                    id: publisher_id,
                                    name: publisher_name,
                                },
                                ..Default::default()
                            },
                            price,
                            ..Default::default()
                        },
                    )
                },
            )
            .await?;

        let items = items.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut items, mut catalogs), (item, catalog)| {
                items.push(item);
                catalogs.push(catalog);
                (items, catalogs)
            },
        );

        match purchase_order.pop() {
            Some(mut purchase_order) => {
                purchase_order.items = items.0;
                Ok(Some((purchase_order, items.1)))
            }
            None => anyhow::bail!("failed to find purchase order"),
        }
    }

    pub async fn check_book_in_purchase_order(
        conn: &mut Conn,
        book_id: u32,
        purchase_order_id: u32,
    ) -> anyhow::Result<bool> {
        let query = r"SELECT COUNT(*) FROM purchase_order_items
        WHERE purchase_order_id = :purchase_order_id AND supplier_catalog_id IN (
            SELECT supplier_catalog_id FROM supplier_catalogs WHERE book_id = :book_id
        )";
        let params = params! {
            "purchase_order_id" => purchase_order_id,
            "book_id" => book_id,
        };
        let count = query.with(params).first::<u32, &mut Conn>(conn).await?;
        Ok(count > Some(0))
    }

    pub async fn check_book_quantity_in_purchase_order(
        conn: &mut Conn,
        book_id: u32,
        purchase_order_id: u32,
        quantity: u32,
    ) -> anyhow::Result<bool> {
        let query = r"SELECT SUM(purchase_order_items.quantity) FROM purchase_order_items
        WHERE purchase_order_id = :purchase_order_id AND supplier_catalog_id IN (
            SELECT supplier_catalog_id FROM supplier_catalogs WHERE book_id = :book_id AND available_quantity >= :quantity
        )";
        let params = params! {
            "purchase_order_id" => purchase_order_id,
            "book_id" => book_id,
            "quantity" => quantity,
        };
        let total_quantity = query.with(params).first::<u32, &mut Conn>(conn).await?;
        Ok(total_quantity > Some(quantity))
    }
}
