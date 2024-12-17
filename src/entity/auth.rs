use mysql_common::time::PrimitiveDateTime;

pub struct AuthRecord {
    pub customer_id: u32,
    pub token: String,
    pub last_used: PrimitiveDateTime,
    pub is_online: bool,
}
