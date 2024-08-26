#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod allocation_vesting {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };

    #[derive(Debug, Clone, PartialEq, Eq, ink_storage::traits::SpreadLayout, ink_storage::traits::PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
    pub struct AllocationSplit {
        recipient: AccountId,
        points: u32,
        number_of_weeks: u8,
    }

    #[derive(Debug, Clone, PartialEq, Eq, ink_storage::traits::SpreadLayout, ink_storage::traits::PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
    pub struct AllocationState {
        points: u32,
        number_of_weeks: u8,
        claimed: Balance,
        preclaimed: Balance,
    }

    #[ink::contract]
    pub struct AllocationVesting {
        allocations: StorageHashMap<AccountId, AllocationState>,
        max_total_preclaim_pct: u32,
        total_allocation: Balance,
        vesting_start: Option<Timestamp>,
        owner: AccountId,
    }

    impl AllocationVesting {
        #[ink(constructor)]
        pub fn new(total_allocation: Balance, max_total_preclaim_pct: u32) -> Self {
            ink_env::debug_assert!(total_allocation > 0);
            ink_env::debug_assert!(max_total_preclaim_pct <= 20);
            Self {
                allocations: StorageHashMap::new(),
                max_total_preclaim_pct,
                total_allocation,
                vesting_start: None,
                owner: Self::env().caller(),
            }
        }

        #[ink(message)]
        pub fn set_allocations(&mut self, allocation_splits: Vec<AllocationSplit>, vesting_start: Timestamp) -> Result<(), ContractError> {
            if self.vesting_start.is_some() {
                return Err(ContractError::VestingAlreadyStarted);
            }
            self.vesting_start = Some(vesting_start);
            // Additional logic to check and set allocations
            Ok(())
        }

        #[ink(message)]
        pub fn lock_allocation(&self, account: AccountId, amount: Balance, weeks: u32) -> Result<(), ContractError> {
            // Locking logic using external token locker
            Ok(())
        }
    }

    #[ink(storage)]
    #[derive(Debug)]
    pub enum ContractError {
        NothingToClaim,
        CannotLock,
        WrongMaxTotalPreclaimPct,
        PreclaimTooLarge,
        AllocationsMismatch,
        ZeroTotalAllocation,
        ZeroAllocation,
        ZeroNumberOfWeeks,
        DuplicateAllocation,
        InsufficientPoints,
        LockedAllocation,
        IllegalVestingStart,
        VestingAlreadyStarted,
        IncompatibleVestingPeriod,
    }
}

fn main() {}