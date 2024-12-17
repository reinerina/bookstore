use crate::entity::{CreditRule, Customer};
use crate::repo::{AuthRepo, UserRepo};
use crate::service::AuthService;
use crate::utils::{encrypt_password, generate_token, validate_token, Token};
use mysql_async::Conn;

pub struct UserService;

impl UserService {
    pub async fn login(conn: &mut Conn, username: &str, password: &str) -> anyhow::Result<Token> {
        let password = encrypt_password(password).await?;

        match UserRepo::login(conn, username, &password).await? {
            Some(user_id) => {
                let token = generate_token(username).await?;
                AuthRepo::create_auth_record(conn, user_id, &token.token).await?;
                Ok(token)
            }
            None => anyhow::bail!("login failed, username or password incorrect"),
        }
    }

    pub async fn register(
        conn: &mut Conn,
        username: &str,
        password: &str,
        name: &str,
    ) -> anyhow::Result<Token> {
        let password = encrypt_password(password).await?;

        match UserRepo::register(conn, username, &password, name).await? {
            Some(user_id) => {
                let token = generate_token(username).await?;
                AuthRepo::create_auth_record(conn, user_id, &token.token).await?;
                Ok(token)
            }

            None => anyhow::bail!("register failed"),
        }
    }

    pub async fn get_user_detail(conn: &mut Conn, token: &Token) -> anyhow::Result<Customer> {
        let (_, username) = AuthService::verify_user(conn, token).await?;

        match UserRepo::get_user_detail(conn, &username).await? {
            Some(user) => Ok(user),
            None => anyhow::bail!("user {} not found", username),
        }
    }

    pub async fn get_user_profile(conn: &mut Conn, username: &str) -> anyhow::Result<Customer> {
        match UserRepo::get_user_detail(conn, &username).await? {
            Some(user) => Ok(Customer {
                username: user.username,
                name: user.name,
                email: user.email,
                credit_level: user.credit_level,
                ..Default::default()
            }),
            None => anyhow::bail!("user {} not found", username),
        }
    }

    pub async fn get_credit_rule(conn: &mut Conn, token: &Token) -> anyhow::Result<CreditRule> {
        let (_, username) = AuthService::verify_user(conn, token).await?;

        let credit_level = match UserRepo::get_user_credit_level(conn, &username).await? {
            Some(credit_level) => credit_level,
            None => anyhow::bail!("user {} not found", username),
        };

        match UserRepo::get_credit_rule(conn, credit_level).await? {
            Some(credit_rule) => Ok(credit_rule),
            None => anyhow::bail!("credit rule {} not found", credit_level),
        }
    }

    pub async fn verify_user(conn: &mut Conn, token: &Token) -> anyhow::Result<(u32, String)> {
        let username = validate_token(token).await?;

        match UserRepo::get_user_id(conn, &username).await? {
            Some(user_id) => Ok((user_id, username)),
            None => anyhow::bail!("user {} not found", username),
        }
    }
}
