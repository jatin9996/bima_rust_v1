use borsh::{BorshSerialize, BorshDeserialize};
use sdk::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Proposal {
    pub week: u16,
    pub created_at: u64,
    pub can_execute_after: u64,
    pub current_weight: u64,
    pub required_weight: u64,
    pub processed: bool,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Action {
    pub target: Pubkey,
    pub data: Vec<u8>,
}
