use crate::entity::Publisher;
use mysql_async::prelude::{Query, WithParams};
use mysql_async::{params, Conn};

pub struct PublisherRepo;

impl PublisherRepo {
    pub async fn get_publisher_list(conn: &mut Conn) -> anyhow::Result<Vec<Publisher>> {
        let query = r"SELECT id, name FROM publishers";
        let result = query
            .map(&mut *conn, |(id, name)| {
                let id: u32 = id;
                let name: String = name;
                Publisher { id, name }
            })
            .await?;
        Ok(result)
    }

    pub async fn get_publisher(
        conn: &mut Conn,
        publisher_id: u32,
    ) -> anyhow::Result<Option<Publisher>> {
        let query = r"SELECT id, name FROM publishers WHERE id = :publisher_id";
        let params = params! {
            "publisher_id" => publisher_id,
        };
        let mut result = query
            .with(params)
            .map(&mut *conn, |(id, name)| {
                let id: u32 = id;
                let name: String = name;
                Publisher { id, name }
            })
            .await?;
        Ok(result.pop())
    }

    pub async fn add_publisher(conn: &mut Conn, name: &str) -> anyhow::Result<Option<u32>> {
        let query = r"INSERT INTO publishers(name) VALUES(:name)";
        let params = params! {
            "name" => name,
        };
        query.with(params).run(&mut *conn).await?;
        let query = r"SELECT LAST_INSERT_ID() as publisher_id";
        let publisher_id = query.with(()).first::<u32, &mut Conn>(conn).await?;
        Ok(publisher_id)
    }

    pub async fn update_publisher(
        conn: &mut Conn,
        publisher_id: u32,
        name: &str,
    ) -> anyhow::Result<Option<u32>> {
        let query = r"UPDATE publishers SET name = :name WHERE id = :publisher_id";
        let params = params! {
            "name" => name,
            "publisher_id" => publisher_id,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(Some(publisher_id))
    }
}
