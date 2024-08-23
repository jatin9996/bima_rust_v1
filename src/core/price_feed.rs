use crate::dependencies::babel_math::BabelMath;
use crate::dependencies::babel_ownable::BabelOwnable;
use crate::interfaces::aggregator_v3::AggregatorV3Interface;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Default, Clone)]
pub struct OracleRecord {
    chainlink_oracle: Box<dyn AggregatorV3Interface>, // Use dynamic dispatch for the oracle
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
    owner: BabelOwnable, // Add ownership management
}

impl PriceFeed {
    pub fn new(owner_account: AccountId) -> Self {
        Self {
            oracle_records: HashMap::new(),
            price_records: HashMap::new(),
            owner: BabelOwnable::new(owner_account), // Initialize the owner
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
        self.owner.only_owner(); // Check ownership
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
        self.price_records.get(token).map(|record| {
            let (round_id, answer, timestamp, _, _) = record.chainlink_oracle.latest_round_data();
            let scaled_price = BabelMath::dec_mul(
                BabelMath::from(answer), 
                BabelMath::from(10u128.pow(u32::from(18 - record.decimals)))
            );
            record.scaled_price = scaled_price;
            record.timestamp = timestamp as u32;
            record.round_id = round_id as u64;
            scaled_price
        })
    }
}