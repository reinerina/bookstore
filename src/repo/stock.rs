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
}
