use crate::entity::Location;
use mysql_async::prelude::{Query, WithParams};
use mysql_async::{params, Conn};

pub struct StockRepo;

impl StockRepo {
    pub async fn get_stock(conn: &mut Conn, location_id: u32) -> anyhow::Result<Vec<(u32, u32)>> {
        let query =
            r"SELECT book_id, quantity FROM book_locations WHERE location_id = :location_id";
        let params = params! {
            "location_id" => location_id,
        };
        let result = query
            .with(params)
            .map(&mut *conn, |(book_id, quantity)| {
                let book_id: u32 = book_id;
                let quantity: u32 = quantity;
                (book_id, quantity)
            })
            .await?;
        Ok(result)
    }

    pub async fn get_book_stock(conn: &mut Conn, book_id: u32) -> anyhow::Result<Vec<(u32, u32)>> {
        let query = r"SELECT location_id, quantity FROM book_locations WHERE book_id = :book_id";
        let params = params! {
            "book_id" => book_id,
        };
        let result = query
            .with(params)
            .map(&mut *conn, |(location_id, quantity)| {
                let location_id: u32 = location_id;
                let quantity: u32 = quantity;
                (location_id, quantity)
            })
            .await?;
        Ok(result)
    }

    pub async fn get_book_quantity(
        conn: &mut Conn,
        book_id: u32,
        location_id: u32,
    ) -> anyhow::Result<Option<u32>> {
        let query = r"SELECT quantity FROM book_locations WHERE book_id = :book_id AND location_id = :location_id";
        let params = params! {
            "book_id" => book_id,
            "location_id" => location_id,
        };
        let mut result = query
            .with(params)
            .map(&mut *conn, |quantity| {
                let quantity: u32 = quantity;
                quantity
            })
            .await?;
        Ok(result.pop())
    }

    pub async fn get_total_book_quantity(
        conn: &mut Conn,
        book_id: u32,
    ) -> anyhow::Result<Option<u32>> {
        let query = r"SELECT SUM(quantity) FROM book_locations WHERE book_id = :book_id";
        let params = params! {
            "book_id" => book_id,
        };
        let mut result = query
            .with(params)
            .map(&mut *conn, |quantity| {
                let quantity: u32 = quantity;
                quantity
            })
            .await?;
        Ok(result.pop())
    }

    pub async fn out_stock(
        conn: &mut Conn,
        book_id: u32,
        location_id: u32,
        quantity: u32,
    ) -> anyhow::Result<()> {
        let query = r"UPDATE book_locations SET quantity = quantity - :quantity WHERE book_id = :book_id AND location_id = :location_id";
        let params = params! {
            "book_id" => book_id,
            "location_id" => location_id,
            "quantity" => quantity,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }

    pub async fn out_stock_automatic(
        conn: &mut Conn,
        book_id: u32,
        quantity: u32,
    ) -> anyhow::Result<()> {
        let query = r"UPDATE book_locations SET quantity = quantity - :quantity WHERE book_id = :book_id AND quantity >= :quantity";
        let params = params! {
            "book_id" => book_id,
            "quantity" => quantity,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }

    pub async fn in_stock(
        conn: &mut Conn,
        book_id: u32,
        location_id: u32,
        quantity: u32,
    ) -> anyhow::Result<()> {
        let query = r"INSERT INTO book_locations(book_id, location_id, quantity) VALUES(:book_id, :location_id, :quantity)
        ON DUPLICATE KEY UPDATE quantity = quantity + :quantity";
        let params = params! {
            "book_id" => book_id,
            "location_id" => location_id,
            "quantity" => quantity,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }

    pub async fn get_location_list(conn: &mut Conn) -> anyhow::Result<Vec<Location>> {
        let query = r"SELECT location_id, description FROM loactions;";
        let result = query
            .with(())
            .map(&mut *conn, |(location_id, description)| {
                let location_id: u32 = location_id;
                Location {
                    id: location_id,
                    description,
                    book_id: 0,
                    quantity: 0,
                }
            })
            .await?;
        Ok(result)
    }

    pub async fn get_location_list_by_book(
        conn: &mut Conn,
        book_id: u32,
    ) -> anyhow::Result<Vec<Location>> {
        let query = r"SELECT DISTINCT
	loactions.location_id,
	loactions.description,
	book_locations.book_id,
	book_locations.quantity
FROM
	book_locations
	LEFT JOIN loactions ON loactions.location_id = book_locations.location_id
WHERE
	book_locations.book_id = :book_id";
        let params = params! {
            "book_id" => book_id,
        };
        let result = query
            .with(params)
            .map(
                &mut *conn,
                |(location_id, description, book_id, quantity)| {
                    let location_id: u32 = location_id;
                    let book_id: u32 = book_id;
                    let quantity: u32 = quantity;
                    Location {
                        id: location_id,
                        description,
                        book_id,
                        quantity,
                    }
                },
            )
            .await?;
        Ok(result)
    }

    pub async fn get_location_detail(
        conn: &mut Conn,
        location_id: u32,
    ) -> anyhow::Result<Option<Location>> {
        let query =
            r"SELECT location_id, description FROM loactions WHERE location_id = :location_id";
        let params = params! {
            "location_id" => location_id,
        };
        let mut result = query
            .with(params)
            .map(&mut *conn, |(location_id, description)| {
                let location_id: u32 = location_id;
                Location {
                    id: location_id,
                    description,
                    book_id: 0,
                    quantity: 0,
                }
            })
            .await?;
        Ok(result.pop())
    }

    pub async fn add_location(conn: &mut Conn, description: &str) -> anyhow::Result<Option<u32>> {
        let query = r"INSERT INTO loactions(description) VALUES(:description)";
        let params = params! {
            "description" => description,
        };
        query.with(params).run(&mut *conn).await?;
        let query = r"SELECT LAST_INSERT_ID() as location_id";
        let location_id = query.with(()).first::<u32, &mut Conn>(conn).await?;
        Ok(location_id)
    }

    pub async fn update_location(
        conn: &mut Conn,
        location_id: u32,
        description: &str,
    ) -> anyhow::Result<Option<u32>> {
        let query =
            r"UPDATE loactions SET description = :description WHERE location_id = :location_id";
        let params = params! {
            "description" => description,
            "location_id" => location_id,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(Some(location_id))
    }

    pub async fn delete_location(conn: &mut Conn, location_id: u32) -> anyhow::Result<()> {
        let query = r"DELETE FROM loactions WHERE location_id = :location_id";
        let params = params! {
            "location_id" => location_id,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }

    pub async fn delete_book_location(
        conn: &mut Conn,
        book_id: u32,
        location_id: u32,
    ) -> anyhow::Result<()> {
        let query =
            r"DELETE FROM book_locations WHERE book_id = :book_id AND location_id = :location_id";
        let params = params! {
            "book_id" => book_id,
            "location_id" => location_id,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }

    pub async fn update_book_location(
        conn: &mut Conn,
        book_id: u32,
        location_id: u32,
        quantity: u32,
    ) -> anyhow::Result<()> {
        let query = r"UPDATE book_locations SET quantity = :quantity WHERE book_id = :book_id AND location_id = :location_id";
        let params = params! {
            "book_id" => book_id,
            "location_id" => location_id,
            "quantity" => quantity,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }
}
