#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod admin_voting {
    use ink_storage::{
        collections::HashMap as StorageMap,
        traits::{PackedLayout, SpreadLayout},
    };

    #[ink(storage)]
    pub struct AdminVoting {
        token_locker: AccountId,
        babel_core: AccountId,
        proposal_data: StorageMap<u32, Proposal>,
        proposal_payloads: StorageMap<u32, Vec<Action>>,
        account_vote_weights: StorageMap<(AccountId, u32), u64>,
        latest_proposal_timestamp: StorageMap<AccountId, Timestamp>,
        min_create_proposal_pct: u32,
        passing_pct: u32,
    }

    #[derive(Debug, Clone, PartialEq, Eq, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Proposal {
        week: u16,
        created_at: Timestamp,
        can_execute_after: Timestamp,
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
        pub fn new(token_locker: AccountId, babel_core: AccountId, min_create_proposal_pct: u32, passing_pct: u32) -> Self {
            Self { 
                token_locker,
                babel_core,
                proposal_data: StorageMap::new(),
                proposal_payloads: StorageMap::new(),
                account_vote_weights: StorageMap::new(),
                latest_proposal_timestamp: StorageMap::new(),
                min_create_proposal_pct,
                passing_pct,
            }
        }

        #[ink(message)]
        pub fn create_new_proposal(&mut self, account: AccountId, payload: Vec<Action>) -> Result<(), ink_env::Error> {
            let current_time = Self::env().block_timestamp();
            let last_proposal_time = self.latest_proposal_timestamp.get(&account).copied().unwrap_or(0);

            // Check if the minimum time between proposals has passed
            assert!(current_time > last_proposal_time + MIN_TIME_BETWEEN_PROPOSALS, "Minimum time between proposals not met");

            // Calculate the week number (assuming a function exists to calculate it)
            let week = self.calculate_week_number(current_time);
            assert!(week > 0, "No proposals in the first week");

            // Get the weight of the account at the previous week
            let account_weight = self.token_locker.get_account_weight_at(account, week - 1);
            let min_weight = self.min_create_proposal_weight(week - 1);

            // Check if the account has enough weight to propose
            assert!(account_weight >= min_weight, "Not enough weight to propose");

            // Create the proposal
            let proposal_id = self.proposal_data.len() as u32; // Incremental ID
            let new_proposal = Proposal {
                week: week as u16,
                created_at: current_time,
                can_execute_after: 0,
                current_weight: 0,
                required_weight: self.calculate_required_weight(week - 1, self.passing_pct),
                processed: false,
            };

            // Store the proposal and its payloads
            self.proposal_data.insert(proposal_id, new_proposal);
            self.proposal_payloads.insert(proposal_id, payload);
            self.latest_proposal_timestamp.insert(account, current_time);

            Ok(())
        }

        // Helper function to calculate the required weight
        fn calculate_required_weight(&self, week: u32, pct: u32) -> u64 {
            let total_weight = self.token_locker.get_total_weight_at(week);
            (total_weight * pct as u64) / 10000
        }

        // Helper function to calculate the minimum weight needed to create a proposal
        fn min_create_proposal_weight(&self, week: u32) -> u64 {
            let total_weight = self.token_locker.get_total_weight_at(week);
            (total_weight * self.min_create_proposal_pct as u64) / 10000
        }

        // Dummy function to calculate week number based on timestamp
        fn calculate_week_number(&self, timestamp: Timestamp) -> u32 {
            // Implementation depends on how weeks are defined in your system
            timestamp / (7 * 24 * 60 * 60) 
        }

        #[ink(message)]
        pub fn vote_for_proposal(&mut self, account: AccountId, id: u32, weight: u64) {
            // Implementation similar to Solidity's voteForProposal
        }
    }
}