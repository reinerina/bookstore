use crate::entity::CustomerStatus;
use mysql_common::bigdecimal::BigDecimal;

#[derive(Debug, Default)]
pub struct Customer {
    pub id: u32,
    pub username: String,
    pub password: String,
    pub name: String,
    pub address: String,
    pub email: String,
    pub account_balance: BigDecimal,
    pub credit_level: u32,
    pub total_purchase: BigDecimal,
    pub overdraft_limit: BigDecimal,
    pub status: CustomerStatus,
}

#[derive(Debug, Default)]
pub struct CreditRule {
    pub level: u32,
    pub discount_percentage: BigDecimal,
    pub overdraft_limit: BigDecimal,
    pub upgrade_balance: BigDecimal,
    pub upgrade_purchase: BigDecimal,
}
