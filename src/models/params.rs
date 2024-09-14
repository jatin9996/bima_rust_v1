use borsh::{BorshSerialize, BorshDeserialize};
use sdk::arch_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DepositParams {
    pub utxo_id: String,
    pub caller_pubkey: Pubkey,
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct WithdrawParams {
    pub utxo_id: String,
    pub caller_pubkey: Pubkey,
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct InterestParams {
    pub collateral_amount: u64,
}
