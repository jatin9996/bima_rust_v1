#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod vault {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };

    #[ink(storage)]
    pub struct Vault {
        babel_token: AccountId,
        emission_schedule: AccountId,
        token_locker: AccountId,
        boost_calculator: AccountId,
        incentive_voting: AccountId,
        babel_ownable: AccountId,
        system_start: AccountId,
        unallocated_total: Balance,
        weekly_emissions: StorageHashMap<u64, Balance>,
        allocated: StorageHashMap<AccountId, Balance>,
    }

    impl Vault {
        #[ink(constructor)]
        pub fn new(
            babel_token: AccountId,
            emission_schedule: AccountId,
            token_locker: AccountId,
            boost_calculator: AccountId,
            incentive_voting: AccountId,
            babel_ownable: AccountId,
            system_start: AccountId,
        ) -> Self {
            Self {
                babel_token,
                emission_schedule,
                token_locker,
                boost_calculator,
                incentive_voting,
                babel_ownable,
                system_start,
                unallocated_total: 0,
                weekly_emissions: StorageHashMap::new(),
                allocated: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn set_weekly_emission(&mut self, week: u64, amount: Balance) {
            let total_emissions = self.get_total_weekly_emissions(week); // Simplified for example
            self.weekly_emissions.insert(week, total_emissions);
            self.unallocated_total -= total_emissions;
            // Example of using the token locker
            self.lock_tokens(lock_amount, 52); // Lock for 1 year
        }

        #[ink(message)]
        pub fn transfer_tokens(&mut self, receiver: AccountId, amount: Balance) {
            // Transfer logic here
            self.unallocated_total -= amount;
        }

        #[ink(message)]
        pub fn increase_unallocated_supply(&mut self, amount: Balance) {
            self.unallocated_total += amount;
            // Increase allowance logic here
        }

        fn get_total_weekly_emissions(&self, week: u64) -> Balance {
            // Calculation logic here
            1000 // Dummy value
        }

        fn lock_tokens(&self, amount: Balance, duration: u64) {
            // Locking logic here
        }
    }
}