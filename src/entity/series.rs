#[derive(Debug, Default)]
pub struct Series {
    pub id: u32,
    pub title: String,
}

#[derive(Debug, Default)]
pub struct BookInSeries {
    pub series_id: u32,
    pub title: String,
    pub column: u32,
}
