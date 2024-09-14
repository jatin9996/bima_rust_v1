use borsh::{BorshSerialize, BorshDeserialize};
use crate::models::{LiquidationParams, LiquidationResult};
use sdk::utxo::UtxoMeta;
use sdk::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct LiquidationManagerInterface;

impl LiquidationManagerInterface {
    pub fn liquidate(&self, params: LiquidationParams) -> LiquidationResult;
    pub fn enable_trove_manager(&self, trove_manager: Pubkey);
}
