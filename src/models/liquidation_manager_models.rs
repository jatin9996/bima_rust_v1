use borsh::{BorshSerialize, BorshDeserialize};
use sdk::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct LiquidationParams {
    pub trove_manager: Pubkey,
    pub borrower: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct LiquidationResult {
    pub borrower: Pubkey,
    pub liquidated_debt: u64,
    pub liquidated_coll: u64,
}
