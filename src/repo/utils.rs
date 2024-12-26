use mysql_async::prelude::{Query, WithParams};
use mysql_async::Conn;
use mysql_common::time::PrimitiveDateTime;

pub struct UtilsRepo;

impl UtilsRepo {
    pub async fn now(conn: &mut Conn) -> anyhow::Result<PrimitiveDateTime> {
        let query = r"SELECT NOW()";
        let result = query
            .with(())
            .first::<PrimitiveDateTime, &mut Conn>(conn)
            .await?;
        match result {
            Some(datetime) => Ok(datetime),
            None => anyhow::bail!("failed to get current datetime"),
        }
    }

    pub async fn transaction(conn: &mut Conn) -> anyhow::Result<()> {
        let query = r"START TRANSACTION;";
        query.run(&mut *conn).await?;
        Ok(())
    }

    pub async fn commit(conn: &mut Conn) -> anyhow::Result<()> {
        let query = r"COMMIT;";
        query.run(&mut *conn).await?;
        Ok(())
    }

    pub async fn rollback(conn: &mut Conn) -> anyhow::Result<()> {
        let query = r"ROLLBACK;";
        query.run(&mut *conn).await?;
        Ok(())
    }
}
