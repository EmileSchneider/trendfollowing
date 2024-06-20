#[derive(Debug)]
pub struct Perpetual {
    pub symbol: String,
}

#[derive(Debug, Clone)]
pub struct Kline {
    pub opentime: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
}
