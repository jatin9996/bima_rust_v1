#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod boost_calculator {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::SpreadAllocate,
    };

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct BoostCalculator {
        locker: AccountId, // Changed to AccountId for ink!
        max_boost_grace_weeks: u32,
        account_weekly_lock_pct: StorageHashMap<(AccountId, u32), u32>,
        total_weekly_weights: StorageHashMap<u32, u64>,
    }

    impl BoostCalculator {
        #[ink(constructor)]
        pub fn new(locker: AccountId, grace_weeks: u32) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.locker = locker;
                contract.max_boost_grace_weeks = grace_weeks + Self::get_week();
                contract.account_weekly_lock_pct = StorageHashMap::new();
                contract.total_weekly_weights = StorageHashMap::new();
            })
        }

        #[ink(message)]
        pub fn get_week() -> u32 {
            let start_date = 1_600_000_000; // start date in UNIX timestamp (seconds)
            let current_time = ink_env::block_timestamp() / 1_000_000_000; // Convert nanoseconds to seconds
            let seconds_per_week = 60 * 60 * 24 * 7;
            ((current_time - start_date) / seconds_per_week) as u32
        }

        #[ink(message)]
        pub fn get_boosted_amount(&self, account: AccountId, amount: u64, previous_amount: u64, total_weekly_emissions: u64) -> u64 {
            let week = Self::get_week();
            if week < self.max_boost_grace_weeks {
                return amount;
            }

            let adjusted_week = week - 1;
            let account_weight = self.get_account_weight(account, adjusted_week).unwrap_or(0);
            let total_weight = self.get_total_weight(adjusted_week).unwrap_or(1);
            let pct = 1_000_000_000 * account_weight / total_weight;

            self.calculate_adjusted_amount(amount, previous_amount, total_weekly_emissions, pct)
        }

        fn get_account_weight(&self, account: AccountId, week: u32) -> Option<u64> {
            self.account_weekly_lock_pct.get(&(account, week)).copied()
        }

        fn get_total_weight(&self, week: u32) -> Option<u64> {
            self.total_weekly_weights.get(&week).copied()
        }

        fn calculate_adjusted_amount(&self, amount: u64, previous_amount: u64, total_weekly_emissions: u64, pct: u64) -> u64 {
            // Calculate the base amount with the current boost percentage
            let base_amount = amount * pct / 1_000_000_000;

            // Adjust the base amount based on the ratio of previous amount to total emissions
            if total_weekly_emissions > 0 {
                let emission_factor = previous_amount * 1_000_000_000 / total_weekly_emissions;
                base_amount * emission_factor / 1_000_000_000
            } else {
                base_amount // If no emissions, return the base amount
            }
        }
    }
}