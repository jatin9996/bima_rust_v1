use crate::models::Action;
use arch_program::pubkey::Pubkey;

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct Proposal {
    pub week: u16,
    pub created_at: u32,
    pub can_execute_after: u32,
    pub current_weight: u64,
    pub required_weight: u64,
    pub processed: bool,
}
