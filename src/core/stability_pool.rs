#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadAllocate,
};

#[ink::contract]
pub mod stability_pool {
    use super::*;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct StabilityPool {
        deposits: StorageHashMap<AccountId, Balance>,
        total_stablecoins: Balance,
        owner: AccountId,
    }

    impl StabilityPool {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.deposits = StorageHashMap::new();
                contract.total_stablecoins = 0;
                contract.owner = Self::env().caller();
            })
        }

        #[ink(message)]
        pub fn deposit(&mut self, amount: Balance) {
            self.only_owner();
            let caller = self.env().caller();
            let current_deposit = self.deposits.entry(caller).or_insert(0);
            *current_deposit += amount;
            self.total_stablecoins += amount;
            // Simulate debt token issuance (logic to be implemented)
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) -> bool {
            self.only_owner();
            let caller = self.env().caller();
            if let Some(current_deposit) = self.deposits.get_mut(&caller) {
                if *current_deposit >= amount {
                    *current_deposit -= amount;
                    self.total_stablecoins -= amount;
                    // Simulate debt token burning (logic to be implemented)
                    return true;
                }
            }
            false
        }

        fn only_owner(&self) {
            assert_eq!(self.owner, self.env().caller(), "Only owner can call this function");
        }

        #[ink(message)]
        pub fn calculate_interest(&self, coll: Balance, debt: Balance) -> Balance {
            let interest_rate: Balance = 5; // Interest rate of 5%
            let rate_decimal: Balance = interest_rate / 100; // Convert to decimal
            coll * rate_decimal * debt // Calculate simple interest
        }
    }
}