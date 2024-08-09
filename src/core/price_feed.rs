#![cfg_attr(not(feature = "std"), no_std)]

pub use ink_lang as ink;

#[ink::contract]
mod price_feed {
    use ink_storage::collections::HashMap as StorageMap;

    #[ink(storage)]
    pub struct PriceFeed {
        oracle_records: StorageMap<AccountId, OracleRecord>,
        price_records: StorageMap<AccountId, PriceRecord>,
    }

    #[derive(Default, Clone, scale::Encode, scale::Decode)]
    pub struct OracleRecord {
        chainlink_oracle: AccountId,
        decimals: u8,
        heartbeat: u32,
        share_price_signature: [u8; 4],
        share_price_decimals: u8,
        is_feed_working: bool,
        is_eth_indexed: bool,
    }

    #[derive(Default, Clone, scale::Encode, scale::Decode)]
    pub struct PriceRecord {
        scaled_price: u128,
        timestamp: u32,
        last_updated: u32,
        round_id: u64,
    }

    #[ink(event)]
    pub struct NewOracleRegistered {
        #[ink(topic)]
        token: AccountId,
        chainlink_aggregator: AccountId,
        is_eth_indexed: bool,
    }

    #[ink(event)]
    pub struct PriceRecordUpdated {
        #[ink(topic)]
        token: AccountId,
        price: u128,
    }

    impl PriceFeed {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                oracle_records: StorageMap::new(),
                price_records: StorageMap::new(),
            }
        }

        #[ink(message)]
        pub fn set_oracle(
            &mut self,
            token: AccountId,
            chainlink_oracle: AccountId,
            heartbeat: u32,
            share_price_signature: [u8; 4],
            share_price_decimals: u8,
            is_eth_indexed: bool,
        ) {
            // Logic to set oracle
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
            self.env().emit_event(NewOracleRegistered {
                token,
                chainlink_aggregator: chainlink_oracle,
                is_eth_indexed,
            });
        }

        #[ink(message)]
        pub fn fetch_price(&self, token: AccountId) -> Option<u128> {
            // Logic to fetch price
            self.price_records.get(&token).map(|record| record.scaled_price)
        }

        // Additional methods for processing feed responses, etc.
    }
}
