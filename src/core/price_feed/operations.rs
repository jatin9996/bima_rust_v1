use borsh::{BorshSerialize, BorshDeserialize};
use crate::contracts::price_feed::data::{OracleRecord, PriceRecord};
use sdk::utxo::UtxoMeta;
use sdk::pubkey::Pubkey;

pub fn set_oracle(utxo: &UtxoMeta, oracle_data: OracleRecord) -> Result<UtxoMeta, String> {
    // Logic to set oracle data in a State UTXO
    // Return new State UTXO
}

pub fn fetch_price(utxo: &UtxoMeta) -> Result<PriceRecord, String> {
    // Logic to fetch price from a State UTXO
    // Return price data
}
