use crate::entity::AdminRole;
use crate::repo::AdminRepo;
use crate::utils::{encrypt_admin_password, generate_token, validate_token, Token};
use mysql_async::Conn;

pub struct AdminService;

impl AdminService {
    pub async fn register(
        conn: &mut Conn,
        username: &str,
        password: &str,
        role: AdminRole,
        token: &Token,
    ) -> anyhow::Result<u32> {
        let password = encrypt_admin_password(password).await?;

        match AdminService::verify_admin(conn, token, AdminRole::Admin).await? {
            (_, _, true) => match AdminRepo::register_admin(conn, username, &password, role).await?
            {
                Some(admin_id) => Ok(admin_id),
                None => anyhow::bail!("register failed"),
            },
            (_, _, false) => anyhow::bail!("permission denied: only admin can register admin user"),
        }
    }

    pub async fn login(conn: &mut Conn, username: &str, password: &str) -> anyhow::Result<Token> {
        let password = encrypt_admin_password(password).await?;

        match AdminRepo::login_admin(conn, username, &password).await? {
            true => {
                let token = generate_token(username).await?;
                Ok(token)
            }
            false => anyhow::bail!("login failed, username or password incorrect"),
        }
    }

    pub async fn verify_admin(
        conn: &mut Conn,
        token: &Token,
        role: AdminRole,
    ) -> anyhow::Result<(u32, String, bool)> {
        let username = validate_token(token).await?;
        match AdminRepo::get_admin(conn, &username).await? {
            Some(admin) => Ok((admin.id, admin.username, admin.role >= role)),
            None => anyhow::bail!("admin {} not found", username),
        }
    }
}
