use crate::entity::AuthRecord;
use mysql_async::prelude::{Query, WithParams};
use mysql_async::{params, Conn};

pub struct AuthRepo;

impl AuthRepo {
    pub async fn create_auth_record(
        conn: &mut Conn,
        customer_id: u32,
        token: &str,
    ) -> anyhow::Result<()> {
        let query = r"INSERT INTO authed_customers (customer_id,token,last_used,is_online)
        VALUE (:customer_id,:token,NOW(),TRUE) ON DUPLICATE KEY UPDATE token=:token,last_used=NOW(),is_online=TRUE;";
        let params = params! {
            "customer_id" => customer_id,
            "token" => token,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }

    pub async fn update_auth_record(
        conn: &mut Conn,
        customer_id: u32,
        token: &str,
        is_online: bool,
    ) -> anyhow::Result<()> {
        let query = r"UPDATE authed_customers SET token=:token,last_used=NOW(),is_online=:is_online
        WHERE customer_id=:customer_id;";
        let params = params! {
            "customer_id" => customer_id,
            "token" => token,
            "is_online" => is_online,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }

    pub async fn get_auth_record(
        conn: &mut Conn,
        customer_id: u32,
    ) -> anyhow::Result<Option<AuthRecord>> {
        let query = r"SELECT customer_id,token,last_used,is_online FROM authed_customers
        WHERE customer_id = :customer_id;";
        let params = params! {
            "customer_id" => customer_id
        };
        let mut result = query
            .with(params)
            .map(conn, |(customer_id, token, last_used, is_online)| {
                AuthRecord {
                    customer_id,
                    token,
                    last_used,
                    is_online,
                }
            })
            .await?;

        Ok(result.pop())
    }

    pub async fn is_online(conn: &mut Conn, customer_id: u32) -> anyhow::Result<Option<bool>> {
        let query = r"SELECT is_online FROM authed_customers WHERE customer_id = :customer_id;";
        let params = params! {
            "customer_id" => customer_id
        };
        let result = query.with(params).first::<bool, &mut Conn>(conn).await?;
        Ok(result)
    }
}
