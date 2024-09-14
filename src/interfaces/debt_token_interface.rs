use borsh::{BorshSerialize, BorshDeserialize};
use arch_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MintParams {
    pub recipient: Pubkey,
    pub amount: u128,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BurnParams {
    pub account: Pubkey,
    pub amount: u128,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TransferParams {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u128,
}
