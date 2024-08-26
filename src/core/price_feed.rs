use ink_lang as ink;
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::{PackedLayout, SpreadLayout},
};
use crate::dependencies::babel_math::BabelMath;
use crate::interfaces::aggregator_v3::AggregatorV3Interface;

#[ink::contract]
mod price_feed {
    use super::*;

    #[ink(storage)]
    pub struct PriceFeed {
        oracle_records: StorageHashMap<String, OracleRecord>,
        price_records: StorageHashMap<String, PriceRecord>,
        owner: AccountId,
    }

    #[derive(Default, Clone, PackedLayout, SpreadLayout)]
    pub struct OracleRecord {
        chainlink_oracle: Box<dyn AggregatorV3Interface>,
        decimals: u8,
        heartbeat: u32,
        share_price_signature: [u8; 4],
        share_price_decimals: u8,
        is_feed_working: bool,
        is_eth_indexed: bool,
    }

    #[derive(Default, Clone, PackedLayout, SpreadLayout)]
    pub struct PriceRecord {
        scaled_price: u128,
        timestamp: u32,
        last_updated: u32,
        round_id: u64,
    }

    impl PriceFeed {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                oracle_records: StorageHashMap::new(),
                price_records: StorageHashMap::new(),
                owner,
            }
        }

        #[ink(message)]
        pub fn set_oracle(
            &mut self,
            token: String,
            chainlink_oracle: Box<dyn AggregatorV3Interface>,
            heartbeat: u32,
            share_price_signature: [u8; 4],
            share_price_decimals: u8,
            is_eth_indexed: bool,
        ) {
            self.only_owner();
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

        #[ink(message)]
        pub fn fetch_price(&self, token: String) -> Option<u128> {
            self.price_records.get(&token).map(|record| {
                let (round_id, answer, timestamp, _, _) = record.chainlink_oracle.latest_round_data();
                let scaled_price = BabelMath::dec_mul(
                    BabelMath::from(answer),
                    BabelMath::from(10u128.pow(u32::from(18 - record.decimals)))
                );
                scaled_price
            })
        }

        fn only_owner(&self) {
            assert_eq!(self.env().caller(), self.owner, "Only owner can call this function");
        }
    }
}