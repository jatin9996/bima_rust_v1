use borsh::{BorshSerialize, BorshDeserialize};
use crate::models::{AllocationSplit, AllocationState};
use crate::errors::ContractError;
use bitcoin::secp256k1::PublicKey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AllocationVesting {
    allocations: Vec<AllocationState>,
    max_total_preclaim_pct: u32,
    total_allocation: u64,
    vesting_start: Option<u64>,  // Timestamp in seconds
    owner: PublicKey,
}

impl AllocationVesting {
    pub fn new(total_allocation: u64, max_total_preclaim_pct: u32, owner: PublicKey) -> Self {
        Self {
            allocations: vec![],
            max_total_preclaim_pct,
            total_allocation,
            vesting_start: None,
            owner,
        }
    }

    pub fn set_allocations(&mut self, allocation_splits: Vec<AllocationSplit>, vesting_start: u64) -> Result<(), ContractError> {
        // Implementation logic here
    }

    pub fn claim(&mut self, account: PublicKey, amount: u64) -> Result<(), ContractError> {
        // Implementation logic here
    }
}
