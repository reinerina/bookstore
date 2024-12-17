use crate::entity::{AdminRole, AdminStatus};

pub struct Admin {
    pub id: u32,
    pub username: String,
    pub password: String,
    pub status: AdminStatus,
    pub role: AdminRole,
}
