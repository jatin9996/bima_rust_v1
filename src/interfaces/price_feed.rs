use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct OracleRecord {
    pub chainlink_oracle: String,
    pub decimals: u8,
    pub share_price_signature: [u8; 4],
    pub share_price_decimals: u8,
    pub is_feed_working: bool,
    pub is_eth_indexed: bool,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct PriceRecord {
    pub scaled_price: u96,
    pub timestamp: u32,
    pub last_updated: u32,
    pub round_id: u80,
}

pub trait IPriceFeed {
    /// Event triggered when a new oracle is registered.
    fn new_oracle_registered(&self, token: &str, chainlink_aggregator: &str, is_eth_indexed: bool);

    /// Event triggered when the price feed status is updated.
    fn price_feed_status_updated(&self, token: &str, oracle: &str, is_working: bool);

    /// Event triggered when the price record is updated.
    fn price_record_updated(&self, token: &str, price: u256);

    /// Fetches the price of the given token.
    fn fetch_price(&self, token: &str) -> u256;

    /// Sets the oracle for a token.
    fn set_oracle(
        &self,
        token: &str,
        chainlink_oracle: &str,
        share_price_signature: [u8; 4],
        share_price_decimals: u8,
        is_eth_indexed: bool
    );

    /// Returns the maximum price deviation from the previous round.
    fn max_price_deviation_from_previous_round(&self) -> u256;

    /// Returns the address of the Babel core contract.
    fn babel_core(&self) -> &str;

    /// Returns the response timeout value.
    fn response_timeout(&self) -> u256;

    /// Returns the target digits for price.
    fn target_digits(&self) -> u256;

    /// Returns the guardian address.
    fn guardian(&self) -> &str;

    /// Returns details about the oracle records for a token.
    fn oracle_records(&self, token: &str) -> OracleRecord;

    /// Returns the owner address.
    fn owner(&self) -> &str;

    /// Returns details about the price records for a token.
    fn price_records(&self, token: &str) -> PriceRecord;
}
