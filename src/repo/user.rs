use crate::entity::{CreditRule, Customer, UserStatus};
use mysql_async::prelude::{Query, WithParams};
use mysql_async::{params, Conn};
use mysql_common::bigdecimal::BigDecimal;

pub struct UserRepo;

impl UserRepo {
    pub async fn login(
        conn: &mut Conn,
        username: &str,
        password: &str,
    ) -> anyhow::Result<Option<u32>> {
        let query = r"SELECT customer_id FROM customers WHERE username=:username AND pwd=:pwd;";
        let params = params! {
            "username" => username,
            "pwd" => password,
        };
        let result = query.with(params).first::<u32, &mut Conn>(conn).await?;
        match result {
            Some(customer_id) => Ok(Some(customer_id)),
            None => anyhow::bail!("login failed, username or password incorrect"),
        }
    }

    pub async fn register(
        conn: &mut Conn,
        username: &str,
        password: &str,
        name: &str,
    ) -> anyhow::Result<Option<u32>> {
        let query = r"INSERT INTO customers (username,pwd,name) VALUES(:username,:pwd,:name);";
        let params = params! {
            "username" => username,
            "pwd" => password,
            "name" => name,
        };
        query.with(params).run(&mut *conn).await?;
        let query = r"SELECT LAST_INSERT_ID() as customer_id;";
        let customer_id = query.with(()).first::<u32, &mut Conn>(conn).await?;
        Ok(customer_id)
    }

    pub async fn get_user_detail(
        conn: &mut Conn,
        username: &str,
    ) -> anyhow::Result<Option<Customer>> {
        let query = r"SELECT customer_id,username,pwd,name,address,email,
        account_balance,credit_level,
        total_purchase,overdraft_limit,status
        FROM customers WHERE username=:username;";
        let params = params! {
            "username" => username,
        };

        let mut result = query
            .with(params)
            .map(
                conn,
                |(
                    customer_id,
                    username,
                    pwd,
                    name,
                    address,
                    email,
                    account_balance,
                    credit_level,
                    total_purchase,
                    overdraft_limit,
                    status,
                )| {
                    Customer {
                        id: customer_id,
                        username,
                        password: pwd,
                        name,
                        address,
                        email,
                        account_balance,
                        credit_level,
                        total_purchase,
                        overdraft_limit,
                        status: {
                            let status: String = status;
                            status.parse().unwrap()
                        },
                    }
                },
            )
            .await?;
        Ok(result.pop())
    }

    pub async fn get_user_id(conn: &mut Conn, username: &str) -> anyhow::Result<Option<u32>> {
        let query = r"SELECT customer_id FROM customers WHERE username=:username;";
        let params = params! {
            "username" => username,
        };
        let result = query.with(params).first::<u32, &mut Conn>(conn).await?;
        Ok(result)
    }

    pub async fn get_user_credit_level(
        conn: &mut Conn,
        username: &str,
    ) -> anyhow::Result<Option<u32>> {
        let query = r"SELECT credit_level FROM customers WHERE username=:username;";
        let params = params! {
            "username" => username,
        };
        let result = query.with(params).first::<u32, &mut Conn>(conn).await?;
        Ok(result)
    }

    pub async fn get_credit_rule(
        conn: &mut Conn,
        credit_level: u32,
    ) -> anyhow::Result<Option<CreditRule>> {
        let query = r"SELECT credit_level,discount_percentage,overdraft_limit,auto_upgrade_balance,auto_upgrade_total_purchase
        FROM credit_rules WHERE credit_level=:credit_level;";
        let params = params! {
            "credit_level" => credit_level,
        };
        let mut result = query
            .with(params)
            .map(
                conn,
                |(
                    credit_level,
                    discount_percentage,
                    overdraft_limit,
                    auto_upgrade_balance,
                    auto_upgrade_total_purchase,
                )| {
                    CreditRule {
                        level: credit_level,
                        discount_percentage,
                        overdraft_limit,
                        upgrade_balance: auto_upgrade_balance,
                        upgrade_purchase: auto_upgrade_total_purchase,
                    }
                },
            )
            .await?;
        Ok(result.pop())
    }

    pub async fn get_user_list(conn: &mut Conn) -> anyhow::Result<Vec<Customer>> {
        let query = r"SELECT customer_id,username,pwd,name,address,email,
        account_balance,credit_level,
        total_purchase,overdraft_limit,status
        FROM customers;";
        let result = query
            .map(
                conn,
                |(
                    customer_id,
                    username,
                    pwd,
                    name,
                    address,
                    email,
                    account_balance,
                    credit_level,
                    total_purchase,
                    overdraft_limit,
                    status,
                )| {
                    Customer {
                        id: customer_id,
                        username,
                        password: pwd,
                        name,
                        address,
                        email,
                        account_balance,
                        credit_level,
                        total_purchase,
                        overdraft_limit,
                        status: {
                            let status: String = status;
                            status.parse().unwrap()
                        },
                    }
                },
            )
            .await?;
        Ok(result)
    }

    pub async fn set_user_balance(
        conn: &mut Conn,
        customer_id: u32,
        balance: BigDecimal,
    ) -> anyhow::Result<()> {
        let query =
            r"UPDATE customers SET account_balance=:balance WHERE customer_id=:customer_id;";
        let params = params! {
            "balance" => balance,
            "customer_id" => customer_id,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }

    pub async fn set_user_credit_level(
        conn: &mut Conn,
        customer_id: u32,
        credit_level: u32,
    ) -> anyhow::Result<()> {
        let query =
            r"UPDATE customers SET credit_level=:credit_level WHERE customer_id=:customer_id;";
        let params = params! {
            "credit_level" => credit_level,
            "customer_id" => customer_id,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }

    pub async fn update_user_profile(
        conn: &mut Conn,
        customer_id: u32,
        new_username: &str,
        name: &str,
        address: &str,
        email: &str,
    ) -> anyhow::Result<()> {
        let query = r"UPDATE customers SET name=:name,address=:address,email=:email,username=:new_username WHERE customer_id=:customer_id;";
        let params = params! {
            "name" => name,
            "address" => address,
            "email" => email,
            "customer_id" => customer_id,
            "new_username" => new_username,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }

    pub async fn search_username_natural(
        conn: &mut Conn,
        username: &str,
    ) -> anyhow::Result<Vec<Customer>> {
        let query = r"SELECT customer_id,username,pwd,name,address,email,
        account_balance,credit_level,
        total_purchase,overdraft_limit,status
        FROM customers WHERE MATCH(username) AGAINST(:username IN NATURAL LANGUAGE MODE);";
        let params = params! {
            "username" => username,
        };
        let result = query
            .with(params)
            .map(
                conn,
                |(
                    customer_id,
                    username,
                    pwd,
                    name,
                    address,
                    email,
                    account_balance,
                    credit_level,
                    total_purchase,
                    overdraft_limit,
                    status,
                )| {
                    Customer {
                        id: customer_id,
                        username,
                        password: pwd,
                        name,
                        address,
                        email,
                        account_balance,
                        credit_level,
                        total_purchase,
                        overdraft_limit,
                        status: {
                            let status: String = status;
                            status.parse().unwrap()
                        },
                    }
                },
            )
            .await?;
        Ok(result)
    }

    pub async fn update_user_credit_level(
        conn: &mut Conn,
        customer_id: u32,
        credit_level: u32,
    ) -> anyhow::Result<()> {
        let query =
            r"UPDATE customers SET credit_level=:credit_level WHERE customer_id=:customer_id;";
        let params = params! {
            "credit_level" => credit_level,
            "customer_id" => customer_id,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }

    pub async fn update_user_status(
        conn: &mut Conn,
        customer_id: u32,
        status: UserStatus,
    ) -> anyhow::Result<()> {
        let query = r"UPDATE customers SET status=:status WHERE customer_id=:customer_id;";
        let params = params! {
            "status" => status.to_string(),
            "customer_id" => customer_id,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }

    pub async fn update_user_password(
        conn: &mut Conn,
        customer_id: u32,
        password: &str,
    ) -> anyhow::Result<()> {
        let query = r"UPDATE customers SET pwd=:password WHERE customer_id=:customer_id;";
        let params = params! {
            "password" => password,
            "customer_id" => customer_id,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }

    pub async fn search_user_by_name_natural(
        conn: &mut Conn,
        name: &str,
    ) -> anyhow::Result<Vec<Customer>> {
        let query = r"SELECT customer_id,username,pwd,name,address,email,
        account_balance,credit_level,
        total_purchase,overdraft_limit,status
        FROM customers WHERE MATCH(name) AGAINST(:name IN NATURAL LANGUAGE MODE)
        ORDER BY MATCH(name) AGAINST(:name IN NATURAL LANGUAGE MODE) DESC;";
        let params = params! {
            "name" => name,
        };
        let result = query
            .with(params)
            .map(
                conn,
                |(
                    customer_id,
                    username,
                    pwd,
                    name,
                    address,
                    email,
                    account_balance,
                    credit_level,
                    total_purchase,
                    overdraft_limit,
                    status,
                )| {
                    Customer {
                        id: customer_id,
                        username,
                        password: pwd,
                        name,
                        address,
                        email,
                        account_balance,
                        credit_level,
                        total_purchase,
                        overdraft_limit,
                        status: {
                            let status: String = status;
                            status.parse().unwrap()
                        },
                    }
                },
            )
            .await?;
        Ok(result)
    }

    pub async fn search_user_by_username_natural(
        conn: &mut Conn,
        username: &str,
    ) -> anyhow::Result<Vec<Customer>> {
        let query = r"SELECT customer_id,username,pwd,name,address,email,
        account_balance,credit_level,
        total_purchase,overdraft_limit,status
        FROM customers WHERE MATCH(username) AGAINST(:username IN NATURAL LANGUAGE MODE)
        ORDER BY MATCH(username) AGAINST(:username IN NATURAL LANGUAGE MODE) DESC;";
        let params = params! {
            "username" => username,
        };
        let result = query
            .with(params)
            .map(
                conn,
                |(
                    customer_id,
                    username,
                    pwd,
                    name,
                    address,
                    email,
                    account_balance,
                    credit_level,
                    total_purchase,
                    overdraft_limit,
                    status,
                )| {
                    Customer {
                        id: customer_id,
                        username,
                        password: pwd,
                        name,
                        address,
                        email,
                        account_balance,
                        credit_level,
                        total_purchase,
                        overdraft_limit,
                        status: {
                            let status: String = status;
                            status.parse().unwrap()
                        },
                    }
                },
            )
            .await?;
        Ok(result)
    }
}
