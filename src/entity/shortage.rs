use mysql_common::time::PrimitiveDateTime;

pub struct Shortage {
    pub id: u32,
    pub registration_date: PrimitiveDateTime,
    pub items: Vec<ShortageItem>,
    pub is_resolved: bool,
}

impl Default for Shortage {
    fn default() -> Self {
        Self {
            id: 0,
            registration_date: PrimitiveDateTime::MIN,
            items: Vec::new(),
            is_resolved: false,
        }
    }
}

pub struct ShortageItem {
    pub id: u32,
    pub shortage_id: u32,
    pub book_id: u32,
    pub supplier_id: u32,
    pub shortage: u32,
}
