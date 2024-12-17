use crate::entity::{Admin, AdminRole};
use mysql_async::prelude::{Query, WithParams};
use mysql_async::{params, Conn};

pub struct AdminRepo;

impl AdminRepo {
    pub async fn register_admin(
        conn: &mut Conn,
        username: &str,
        password: &str,
        role: AdminRole,
    ) -> anyhow::Result<Option<u32>> {
        let query =
            r"INSERT INTO admins(admin_username,admin_pwd,role) VALUES(:username,:password,:role);";
        let params = params! {
            "username" => username,
            "password" => password,
            "role" => role.to_string(),
        };
        query.with(params).run(&mut *conn).await?;
        let query = r"SELECT LAST_INSERT_ID() as admin_id;";
        let admin_id = query.with(()).first::<u32, &mut Conn>(conn).await?;
        Ok(admin_id)
    }

    pub async fn login_admin(
        conn: &mut Conn,
        username: &str,
        password: &str,
    ) -> anyhow::Result<bool> {
        let query = r"SELECT COUNT(admin_id) FROM admins WHERE admin_username=:username AND admin_pwd=:password;";
        let params = params! {
            "username" => username,
            "password" => password,
        };
        let result = query.with(params).first::<u32, &mut Conn>(conn).await?;
        Ok(result == Some(1))
    }

    pub async fn get_admin(conn: &mut Conn, admin_username: &str) -> anyhow::Result<Option<Admin>> {
        let query = r"SELECT admin_id,admin_username,status,role FROM admins WHERE admin_username=:admin_username;";
        let params = params! {
            "admin_username" => admin_username,
        };
        let mut result = query
            .with(params)
            .map(conn, |(admin_id, admin_username, status, role)| {
                let status: String = status;
                let role: String = role;
                Admin {
                    id: admin_id,
                    username: admin_username,
                    password: String::new(),
                    status: status.parse().unwrap(),
                    role: role.parse().unwrap(),
                }
            })
            .await?;

        Ok(result.pop())
    }
}
