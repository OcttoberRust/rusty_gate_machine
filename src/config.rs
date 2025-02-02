use std::env;

pub struct Config {
    pub event_name: String,
    pub max_price: f64,
    pub time_interval: u64,
    pub auto_purchase: bool,
}
