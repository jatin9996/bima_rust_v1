#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incentive_voting {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };

    #[ink(storage)]
    pub struct IncentiveVoting {
        token_locker: AccountId,
        vault: AccountId,
        account_lock_data: StorageHashMap<AccountId, AccountData>,
        receiver_count: u128,
        receiver_decay_rate: Vec<u32>,
        receiver_updated_week: Vec<u16>,
        receiver_weekly_weights: Vec<Vec<u64>>,
        receiver_weekly_unlocks: Vec<Vec<u32>>,
        total_decay_rate: u32,
        total_updated_week: u16,
        total_weekly_weights: Vec<u64>,
        total_weekly_unlocks: Vec<u32>,
        delegated_ops: AccountId,
        system_start: u64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Eq, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct AccountData {
        week: u16,
        frozen_weight: u64,
        points: u16,
        lock_length: u8,
        vote_length: u16,
        active_votes: Vec<(u16, u16)>,
        locked_amounts: Vec<u32>,
        weeks_to_unlock: Vec<u8>,
    }

    impl IncentiveVoting {
        #[ink(constructor)]
        pub fn new(token_locker: AccountId, vault: AccountId, delegated_ops: AccountId, system_start: u64) -> Self {
            Self {
                token_locker,
                vault,
                account_lock_data: Default::default(),
                receiver_count: 0,
                receiver_decay_rate: vec![0; 65535],
                receiver_updated_week: vec![0; 65535],
                receiver_weekly_weights: vec![vec![0; 65535]; 65535],
                receiver_weekly_unlocks: vec![vec![0; 65535]; 65535],
                total_decay_rate: 0,
                total_updated_week: 0,
                total_weekly_weights: vec![0; 65535],
                total_weekly_unlocks: vec![0; 65535],
                delegated_ops,
                system_start,
            }
        }

        #[ink(message)]
        pub fn register_account_weight(&mut self, account: AccountId, min_weeks: u64) {
            // Ensure caller or delegated
            // Get lock data
            // Update account data
        }

        #[ink(message)]
        pub fn vote(&mut self, account: AccountId, votes: Vec<Vote>, clear_previous: bool) {
            // Voting logic
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Vote {
        id: u128,
        points: u128,
    }
}