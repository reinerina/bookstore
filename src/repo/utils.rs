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
}
