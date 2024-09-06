#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod admin_voting {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };

    #[ink(storage)]
    pub struct AdminVoting {
        token_locker: AccountId,
        babel_core: AccountId,
        proposal_data: StorageHashMap<u32, Proposal>,
        proposal_payloads: StorageHashMap<u32, Vec<Action>>,
        account_vote_weights: StorageHashMap<(AccountId, u32), u64>,
        latest_proposal_timestamp: StorageHashMap<AccountId, u64>,
        min_create_proposal_pct: u32,
        passing_pct: u32,
        system_start: u64,
    }

    #[derive(Debug, Clone, PartialEq, Eq, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Proposal {
        week: u16,
        created_at: u64,
        can_execute_after: u64,
        current_weight: u64,
        required_weight: u64,
        processed: bool,
    }

    #[derive(Debug, Clone, PartialEq, Eq, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Action {
        target: AccountId,
        data: Vec<u8>,
    }

    impl AdminVoting {
        #[ink(constructor)]
        pub fn new(token_locker: AccountId, babel_core: AccountId, min_create_proposal_pct: u32, passing_pct: u32, system_start: u64) -> Self {
            Self {
                token_locker,
                babel_core,
                proposal_data: StorageHashMap::new(),
                proposal_payloads: StorageHashMap::new(),
                account_vote_weights: StorageHashMap::new(),
                latest_proposal_timestamp: StorageHashMap::new(),
                min_create_proposal_pct,
                passing_pct,
                system_start,
            }
        }

        #[ink(message)]
        pub fn create_new_proposal(&mut self, account: AccountId, payload: Vec<Action>) {
            let current_time = self.env().block_timestamp();
            let last_proposal_time = *self.latest_proposal_timestamp.get(&account).unwrap_or(&0);

            if current_time <= last_proposal_time + Self::min_time_between_proposals() {
                ink_env::panic("Minimum time between proposals not met");
            }

            let week = self.system_start; 
            if week == 0 {
                ink_env::panic("No proposals in the first week");
            }

            let account_weight = 1000; 
            let min_weight = 500; 

            if account_weight < min_weight {
                ink_env::panic("Not enough weight to propose");
            }

            let proposal_id = self.proposal_data.len() as u32;
            let new_proposal = Proposal {
                week: week as u16,
                created_at: current_time,
                can_execute_after: 0,
                current_weight: 0,
                required_weight: 1000, 
                processed: false,
            };

            self.proposal_data.insert(proposal_id, new_proposal);
            self.proposal_payloads.insert(proposal_id, payload);
            self.latest_proposal_timestamp.insert(account, current_time);
        }

        fn min_time_between_proposals() -> u64 {
            24 * 60 * 60 // 24 hours
        }
    }
}