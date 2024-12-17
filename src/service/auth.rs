use crate::repo::{AuthRepo, UtilsRepo};
use crate::service::UserService;
use crate::utils::Token;
use mysql_async::Conn;
use mysql_common::time::Duration;

pub struct AuthService;

impl AuthService {
    pub async fn verify_user(conn: &mut Conn, token: &Token) -> anyhow::Result<(u32, String)> {
        let (customer_id, username) = UserService::verify_user(conn, token).await?;
        match AuthRepo::get_auth_record(conn, customer_id).await? {
            Some(record) => match record.is_online {
                true => match record.token == token.token {
                    true => {
                        let now = UtilsRepo::now(conn).await?;
                        let diff = now - record.last_used;
                        if diff > Duration::minutes(30) {
                            AuthRepo::update_auth_record(conn, customer_id, &token.token, false)
                                .await?;
                            anyhow::bail!("token expired");
                        }
                        AuthRepo::update_auth_record(conn, customer_id, &token.token, true).await?;
                        Ok((customer_id, username))
                    }
                    false => anyhow::bail!("token mismatch"),
                },
                false => anyhow::bail!("user {} is offline", username),
            },
            None => anyhow::bail!("auth record not found"),
        }
    }

    pub async fn logout_user(conn: &mut Conn, token: &Token) -> anyhow::Result<()> {
        let (customer_id, _) = AuthService::verify_user(conn, token).await?;
        AuthRepo::update_auth_record(conn, customer_id, &token.token, false).await?;
        Ok(())
    }
}
