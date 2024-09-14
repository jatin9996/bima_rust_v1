use borsh::{BorshSerialize, BorshDeserialize};
use sdk::arch_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Proposal {
    pub created_at: u64,
    pub can_execute_after: u64,
    pub processed: bool,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Action {
    pub target: Pubkey,
    pub data: Vec<u8>,
}