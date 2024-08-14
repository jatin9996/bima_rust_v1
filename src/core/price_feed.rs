use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Default, Clone)]
pub struct OracleRecord {
    chainlink_oracle: String, // Assuming AccountId is a string
    decimals: u8,
    heartbeat: u32,
    share_price_signature: [u8; 4],
    share_price_decimals: u8,
    is_feed_working: bool,
    is_eth_indexed: bool,
}

#[derive(Default, Clone)]
pub struct PriceRecord {
    scaled_price: u128,
    timestamp: u32,
    last_updated: u32,
    round_id: u64,
}

pub struct PriceFeed {
    oracle_records: HashMap<String, OracleRecord>, // Key is now a String
    price_records: HashMap<String, PriceRecord>, // Key is now a String
}

impl PriceFeed {
    pub fn new() -> Self {
        Self {
            oracle_records: HashMap::new(),
            price_records: HashMap::new(),
        }
    }

    pub fn set_oracle(
        &mut self,
        token: String, // Changed AccountId to String
        chainlink_oracle: String,
        heartbeat: u32,
        share_price_signature: [u8; 4],
        share_price_decimals: u8,
        is_eth_indexed: bool,
    ) {
        let record = OracleRecord {
            chainlink_oracle,
            decimals: 18, // Example value
            heartbeat,
            share_price_signature,
            share_price_decimals,
            is_feed_working: true,
            is_eth_indexed,
        };

        self.oracle_records.insert(token, record);
    }

    pub fn fetch_price(&self, token: &str) -> Option<u128> {
        self.price_records.get(token).map(|record| record.scaled_price)
    }
}