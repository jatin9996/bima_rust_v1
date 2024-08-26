#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod interim_admin {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };

    const MIN_TIME_TO_EXECUTION: u64 = 86400; // 1 day in seconds
    const MAX_TIME_TO_EXECUTION: u64 = 1814400; // 3 weeks in seconds
    const MAX_DAILY_PROPOSALS: u32 = 3;

    #[derive(Debug, Clone, PartialEq, Eq, ink_storage::traits::SpreadLayout, ink_storage::traits::PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
    pub struct Action {
        target: AccountId,
        data: Vec<u8>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct Proposal {
        created_at: u64,
        can_execute_after: u64,
        processed: bool,
    }

    #[ink(storage)]
    pub struct InterimAdmin {
        babel_core: AccountId,
        proposals: StorageHashMap<u64, Proposal>,
        proposal_payloads: StorageHashMap<u64, Vec<Action>>,
        daily_proposals_count: StorageHashMap<u64, u32>,
    }

    impl InterimAdmin {
        #[ink(constructor)]
        pub fn new(babel_core: AccountId) -> Self {
            Self {
                babel_core,
                proposals: StorageHashMap::new(),
                proposal_payloads: StorageHashMap::new(),
                daily_proposals_count: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn create_new_proposal(&mut self, payload: Vec<Action>) {
            let current_time = Self::env().block_timestamp();
            let day = current_time / 86400;
            let current_daily_count = *self.daily_proposals_count.get(&day).unwrap_or(&0);
            assert!(current_daily_count < MAX_DAILY_PROPOSALS, "MAX_DAILY_PROPOSALS reached");

            let proposal_index = self.proposals.len() as u64;
            self.proposals.insert(proposal_index, Proposal {
                created_at: current_time,
                can_execute_after: current_time + MIN_TIME_TO_EXECUTION,
                processed: false,
            });
            self.proposal_payloads.insert(proposal_index, payload);
            self.daily_proposals_count.insert(day, current_daily_count + 1);
        }

        // Additional methods like `execute_proposal`, `cancel_proposal`, etc.
    }
}