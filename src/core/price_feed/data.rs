use borsh::{BorshSerialize, BorshDeserialize};
use sdk::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct OracleRecord {
    pub chainlink_oracle: Pubkey,
    pub decimals: u8,
    pub heartbeat: u32,
    pub share_price_signature: [u8; 4],
    pub share_price_decimals: u8,
    pub is_feed_working: bool,
    pub is_eth_indexed: bool,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct PriceRecord {
    pub scaled_price: u128,
    pub timestamp: u32,
    pub last_updated: u32,
    pub round_id: u64,
}
