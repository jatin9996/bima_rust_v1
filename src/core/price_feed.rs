use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Assuming a simplified interface for AggregatorV3Interface
trait AggregatorV3Interface {
    fn latest_round_data(&self) -> (u64, i128, u32, u32, u32);
}

#[derive(Default, Clone)]
pub struct OracleRecord {
    chainlink_oracle: Box<dyn AggregatorV3Interface>,
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
    oracle_records: HashMap<String, OracleRecord>,
    price_records: HashMap<String, PriceRecord>,
    owner: String, // Changed to String for simplicity
}

impl PriceFeed {
    pub fn new(owner: String) -> Self {
        Self {
            oracle_records: HashMap::new(),
            price_records: HashMap::new(),
            owner,
        }
    }

    pub fn set_oracle(
        &mut self,
        token: String,
        chainlink_oracle: Box<dyn AggregatorV3Interface>,
        heartbeat: u32,
        share_price_signature: [u8; 4],
        share_price_decimals: u8,
        is_eth_indexed: bool,
    ) {
        let record = OracleRecord {
            chainlink_oracle,
            decimals: 18,
            heartbeat,
            share_price_signature,
            share_price_decimals,
            is_feed_working: true,
            is_eth_indexed,
        };
        self.oracle_records.insert(token, record);
    }

    pub fn fetch_price(&mut self, token: &str) -> Option<u128> {
        let current_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;
        let price_record = self.price_records.get(token)?;

        if price_record.last_updated == current_timestamp {
            // Return cached price if it was updated in the current block
            return Some(price_record.scaled_price);
        }

        let oracle_record = self.oracle_records.get(token)?;
        let (round_id, answer, timestamp, _, _) = oracle_record.chainlink_oracle.latest_round_data();

        if self.is_price_stale(timestamp, oracle_record.heartbeat) {
            return None;
        }

        let mut scaled_price = (answer as u128) * 10u128.pow(18 - oracle_record.decimals as u32);

        if oracle_record.share_price_signature != [0; 4] {
            // Simulate fetching share price (this would be a call to another contract in Solidity)
            let share_price = 1u128; // Placeholder for actual share price fetching logic
            scaled_price = (scaled_price * share_price) / 10u128.pow(oracle_record.share_price_decimals as u32);
        }

        if oracle_record.is_eth_indexed {
            // Convert ETH price to USD
            let eth_price = self.fetch_price("ETH")?;
            scaled_price = (scaled_price * eth_price) / 1_000_000_000_000_000_000u128;
        }

        self.store_price(token, scaled_price, timestamp, round_id);
        Some(scaled_price)
    }

    fn is_price_stale(&self, price_timestamp: u32, heartbeat: u32) -> bool {
        let current_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;
        current_timestamp - price_timestamp > heartbeat + 3600 // RESPONSE_TIMEOUT_BUFFER equivalent
    }

    fn store_price(&mut self, token: &str, price: u128, timestamp: u32, round_id: u64) {
        self.price_records.insert(
            token.to_string(),
            PriceRecord {
                scaled_price: price,
                timestamp,
                last_updated: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32,
                round_id,
            },
        );
    }

    fn only_owner(&self) {
        assert_eq!(self.owner, "ExpectedOwner", "Only owner can call this function");
    }
}