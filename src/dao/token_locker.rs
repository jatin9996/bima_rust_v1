#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod token_locker {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::SpreadAllocate,
    };

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct TokenLocker {
        lock_to_token_ratio: u64,
        total_decay_rate: u32,
        total_updated_week: u16,
        account_data: StorageHashMap<AccountId, AccountData>,
    }

    #[derive(Default)]
    pub struct AccountData {
        locked: u32,
        unlocked: u32,
        frozen: u32,
        week: u16,
        update_weeks: Vec<u32>,
    }

    impl TokenLocker {
        #[ink(constructor)]
        pub fn new(lock_to_token_ratio: u64) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.lock_to_token_ratio = lock_to_token_ratio;
                contract.account_data = StorageHashMap::new();
            })
        }

        #[ink(message)]
        pub fn lock(&mut self, account: AccountId, amount: u32, weeks: u16) {
            let account_data = self.account_data.entry(account).or_insert(AccountData {
                locked: 0,
                unlocked: 0,
                frozen: 0,
                week: 0,
                update_weeks: vec![0; 256], // Adjust size as needed
            });

            account_data.locked += amount;
            account_data.week = weeks;
            account_data.update_weeks[weeks as usize] += 1;
            // Add token transfer logic here
        }

        #[ink(message)]
        pub fn unlock(&mut self, account: AccountId) {
            if let Some(account_data) = self.account_data.get_mut(&account) {
                account_data.unlocked += account_data.locked;
                account_data.locked = 0;
                // Further implementation needed here to handle unlock logic
            }
        }

        #[ink(message)]
        pub fn get_account_balances(&self, account: AccountId) -> (u32, u32) {
            if let Some(account_data) = self.account_data.get(&account) {
                (account_data.locked, account_data.unlocked)
            } else {
                (0, 0)
            }
        }
    }
}