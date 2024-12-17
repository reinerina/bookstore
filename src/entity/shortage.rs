use mysql_common::time::PrimitiveDateTime;

pub struct Shortage {
    pub id: u32,
    pub registration_date: PrimitiveDateTime,
    pub item: Vec<ShortageItem>,
    pub is_resolved: bool,
}

pub struct ShortageItem {
    pub id: u32,
    pub shortage_id: u32,
    pub book_id: u32,
    pub supplier_id: u32,
    pub shortage: u32,
}
