#[derive(Debug, Default)]
pub struct Location {
    pub id: u32,
    pub description: String,
}

#[derive(Debug, Default)]
pub struct Stock {
    pub book_id: u32,
    pub locations: Vec<Location>,
    pub quantity: u32,
}
